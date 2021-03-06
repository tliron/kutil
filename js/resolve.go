package js

import (
	"path/filepath"

	urlpkg "github.com/tliron/kutil/url"
)

type ResolveFunc func(id string, raw bool) (urlpkg.URL, error)

type CreateResolverFunc func(url urlpkg.URL, context *Context) ResolveFunc

func NewDefaultResolverCreator(urlContext *urlpkg.Context, path []urlpkg.URL, defaultExtension string) CreateResolverFunc {
	return func(url urlpkg.URL, context *Context) ResolveFunc {
		var origins []urlpkg.URL

		if url != nil {
			origins = append([]urlpkg.URL{url.Origin()}, path...)
		} else {
			origins = path
		}

		if defaultExtension == "" {
			return func(id string, raw bool) (urlpkg.URL, error) {
				return urlpkg.NewValidURL(id, origins, urlContext)
			}
		} else {
			defaultExtension_ := "." + defaultExtension
			return func(id string, raw bool) (urlpkg.URL, error) {
				if !raw {
					if filepath.Ext(id) == "" {
						id += defaultExtension_
					}
				}

				return urlpkg.NewValidURL(id, origins, urlContext)
			}
		}
	}
}
