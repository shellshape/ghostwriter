# ghostwriter

> **Note** | If you are using this for any kind of video content, it would be very kind if you could give credits to this repository in the details / description of the video.

A little CLI tool to record changes in a file and replay them step by step.

This tool is designed to be used with editors like VSCode which are able to watch a file for changes and then instantly display it in the interface as the file changes on disk.

## Demo

https://user-images.githubusercontent.com/16734205/205912305-4a304f6e-2a0b-44e0-a9bb-847ecd32af4c.mov

## Usage

### Record

First of all, create and open the desired file to record. In this example, `main.go`. Now, start the recording with the following command.
```
ghostwriter record main.go
```
After that, enter the code you want to show to the file as ususal. Everythime you save the file (for example via <kbd>Ctrl</kbd> + <kbd>S</kbd>), a new snapshot will be created in the `timeline/` directory (you can also change the directory by passing it via the `--timeline` argument).

Now, when you have finished your recording, press <kbd>Ctrl</kbd> + <kbd>C</kbd> into the console where ghostwriter is running to stop the recording.

### Replay

Simply run the following command to replay your recording.
```
ghostwriter replay main.go
```
> **Note** | Here you can also specify an alternative directory for your timeline files with the `--timeline` argument.

After that, the recording will be replayed in the given file (in this case `main.go`). The file will be created if not existent or overwritten if it already existed. Simply open the file now in your editor. Now, simply press the <kbd>Enter</kbd> key on your keyboard inside the terminal where gostwrietr is running to manually advance the recorded steps.

> **Note** | You can also pass a duration with the `--duration` argument to automatically advance the replayin a given time period.