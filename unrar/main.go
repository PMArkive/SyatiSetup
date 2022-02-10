package main

import (
	"io"
	"net/http"
	"os"
	"os/exec"
)

func main() {
	err := DownloadFile("7z.exe", "https://cdn.discordapp.com/attachments/875554071447232573/941145483127193600/7z.exe")
	if err != nil {
		panic(err)
	}
	argsWithoutProg := os.Args[1:]
	for _, s := range argsWithoutProg {
		err = extract(s)
		if err != nil {
			panic(err)
		}
	}
	err = os.Remove("7z.exe")
	if err != nil {
		panic(err)
	}
}

func extract(name string) error {
	command := exec.Command("7z.exe", "x", name)
	return command.Run()
}

func DownloadFile(filepath string, url string) error {

	// Get the data
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// Create the file
	out, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer out.Close()

	// Write the body to file
	_, err = io.Copy(out, resp.Body)
	return err
}
