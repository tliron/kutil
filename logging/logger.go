package logging

//
// Logger
//

type Logger interface {
	Critical(message string)
	Criticalf(format string, values ...interface{})
	Error(message string)
	Errorf(format string, values ...interface{})
	Warning(message string)
	Warningf(format string, values ...interface{})
	Notice(message string)
	Noticef(format string, values ...interface{})
	Info(message string)
	Infof(format string, values ...interface{})
	Debug(message string)
	Debugf(format string, values ...interface{})
}
