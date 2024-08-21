package main

import (
	"github.com/golemcloud/golem-go/std"

	"demo/archive/archive"
)

func init() {
	archive.SetExportsDemoArchiveApi(&ArchiveImpl{})
}

// archived State can be stored in global variables
var archived []archive.ExportsDemoArchiveApiArchivedList

type ArchiveImpl struct {
}

// Implementation of the exported interface

func (e *ArchiveImpl) Store(name string, items []string) {
	std.Init(std.Packages{Os: true, NetHttp: true})

	archived = append(archived, archive.ExportsDemoArchiveApiArchivedList{
		Name:  name,
		Items: items,
	})
}

func (e *ArchiveImpl) GetAll() []archive.ExportsDemoArchiveApiArchivedList {
	std.Init(std.Packages{Os: true, NetHttp: true})

	return archived
}

func main() {
}
