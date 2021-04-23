# Python script that used ffmpeg to convert all mkv files in a dir to mp4 files
 
from datetime import datetime
import os
 
############ VARS ############
command = 'ffmpeg -i "{in}" -c:v copy -c:a libfdk_aac -b:a 128k "{out}"'
DEBUG = True
COLOR = True
 
ColorCodes = {'black': '30', 'red': '31', 'yellow': '33', 'green': '32', 'blue': '34',
                  'cyan': '36', 'magenta': '35', 'white': '37', 'gray': '90', 'reset': '0'}
 
######### FUNCTIONS #########
 
 
def colored(text, color):
    if not COLOR: return text
    return '\033[' + ColorCodes[str(color).lower()] + 'm' + str(text) + "\033[0m"
 
 
def DebugPrint(Category, Text, Color):
    if not DEBUG:
        return
    print(colored('['+datetime.now().strftime("%H:%M:%S")+'] ', 'yellow') +
          colored('['+Category+'] ', 'magenta')+colored(Text, Color))
 
def getDirFiles(dir, fileEnd):
    files = []
    for file in os.listdir(dir):
            if file.endswith(fileEnd):
                DebugPrint("FindFiles", f"Found: {colored(file, 'blue')}", 'cyan')
                files.append(file)
    return files
 
def ffmpegConvert(file, outFile, command):
    DebugPrint('FFMPEG', f"Starting Convert on {colored(file, 'blue')}", 'cyan')
    command = command.replace('{in}', file).replace('{out}', outFile)
    res = os.system(command)
    if res == 0:
        DebugPrint('FFMPEG', f"Complete!", 'green')
        return 0
    DebugPrint('FFMPEG', f"Failed : /", 'red')
    return 1
 
####### MAIN FUNCTION #######
 
def main():
    errors = 0
    files = getDirFiles('.', '.mkv')
    for file in files:
        outFile = f"{file.split('.')[0]}.mp4"
        errors += ffmpegConvert(file, outFile, command)
    DebugPrint('Main', f'Exiting... Completed with {colored(errors, "red")} Errors', 'red')
 
 
if __name__ == "__main__":
    DebugPrint('Main', 'Starting...', 'green')
    main()