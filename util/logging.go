package util

import (
	"fmt"
	"io/ioutil"
	"os"
	"runtime"
	"strings"

	"github.com/op/go-logging"
	"github.com/tliron/kutil/terminal"
)

var plainFormatter = logging.MustStringFormatter(
	`%{time:2006/01/02 15:04:05.000} %{level:8.8s} [%{module}] %{message}`,
)

var colorFormatter = logging.MustStringFormatter(
	`%{color}%{time:2006/01/02 15:04:05.000} %{level:8.8s} [%{module}] %{message}%{color:reset}`,
)

const logFileWritePermissions = 0600

func ConfigureLogging(verbosity int, path *string) {
	var backend *logging.LogBackend

	if verbosity == -1 {
		backend = logging.NewLogBackend(ioutil.Discard, "", 0)
	} else {
		if path != nil {
			if file, err := os.OpenFile(*path, os.O_WRONLY|os.O_CREATE|os.O_APPEND, logFileWritePermissions); err == nil {
				// defer f.Close() ???
				backend = logging.NewLogBackend(file, "", 0)
				logging.SetFormatter(plainFormatter)
			} else {
				Failf("log file error: %s", err.Error())
			}
		} else {
			backend = logging.NewLogBackend(terminal.Stderr, "", 0)
			if terminal.Colorize {
				logging.SetFormatter(colorFormatter)
			} else {
				logging.SetFormatter(plainFormatter)
			}
		}

		verbosity += 3 // our 0 verbosity is NOTICE (3)
		if verbosity > 5 {
			verbosity = 5
		}
	}

	leveledBackend := logging.AddModuleLevel(backend)

	level := logging.Level(verbosity)

	leveledBackend.SetLevel(level, "")

	logging.SetBackend(NewPrefixLeveledBackend(leveledBackend))
}

func LogStack(log *logging.Logger, message string, skip int) {
	// See: https://golang.org/src/runtime/example_test.go

	var builder strings.Builder
	fmt.Fprintf(&builder, "%s\n", message)

	pc := make([]uintptr, 30)
	if n := runtime.Callers(skip+2, pc); n > 0 {
		pc = pc[:n]
		frames := runtime.CallersFrames(pc)
		for {
			frame, more := frames.Next()
			fn := runtime.FuncForPC(frame.PC)
			fmt.Fprintf(&builder, "%s\n  %s:%d\n", fn.Name(), frame.File, frame.Line)

			if !more {
				break
			}
		}
	}

	log.Critical(strings.TrimRight(builder.String(), "\n"))
}

//
// PrefixLeveledBackend
//

type PrefixLeveledBackend struct {
	wrapped      logging.LeveledBackend
	prefixLevels map[string]logging.Level
}

func NewPrefixLeveledBackend(wrapped logging.LeveledBackend) *PrefixLeveledBackend {
	return &PrefixLeveledBackend{
		wrapped:      wrapped,
		prefixLevels: make(map[string]logging.Level),
	}
}

// logging.Leveled interface
func (self *PrefixLeveledBackend) GetLevel(module string) logging.Level {
	for prefix, level := range self.prefixLevels {
		if strings.HasPrefix(module, prefix) {
			return level
		}
	}
	return self.wrapped.GetLevel(module)
}

func (self *PrefixLeveledBackend) SetLevel(level logging.Level, module string) {
	if strings.HasSuffix(module, "*") {
		self.prefixLevels[module[:len(module)-1]] = level
	} else {
		self.wrapped.SetLevel(level, module)
	}
}

func (self *PrefixLeveledBackend) IsEnabledFor(level logging.Level, module string) bool {
	return level <= self.GetLevel(module)
}

// logging.Backend interface
func (self *PrefixLeveledBackend) Log(level logging.Level, callDepth int, record *logging.Record) error {
	return self.wrapped.Log(level, callDepth, record)
}
