package main

import (
	"context"
	"os"
	"path"
	"strings"

	"github.com/mholt/archiver/v4"
)

func main() {
	argsWithoutProg := os.Args[1:]
	for _, s := range argsWithoutProg {
		name := FilenameWithoutExtension(s)
		extract(s, name)
	}
}

func extract(file string, name string) {
	data, err := os.Open(file)
	if err != nil {
		panic(err)
	}
	err = os.Mkdir(name, os.ModePerm)
	if err != nil {
		panic(err)
	}
	format := archiver.Rar{}
	ctx := context.Background()
	handel := func(ctx context.Context, f archiver.File) error {
		data, err := f.Open()
		if err != nil {
			return err
		}
		buffer := make([]byte, f.Size())
		_, err = data.Read(buffer)
		if err != nil {
			return err
		}
		aname := name
		aname = aname + "\\"
		aname = aname + f.Name()
		err = os.WriteFile(aname, buffer, os.ModePerm)
		if err != nil {
			return err
		}
		return nil
	}
	format.Extract(ctx, data, nil, handel)
}

func FilenameWithoutExtension(fn string) string {
	return strings.TrimSuffix(fn, path.Ext(fn))
}
