# Easily genareate a markdown table with file hashes for github :P

from datetime import datetime
import hashlib
import sys

############ VARS ############
COLOR = True # Print With COLOR (Very Cool)
BOLD  = True # When printing with color make text bold

# Define what hashmethoods you want to use!
hashMethoods = {'MD5':    hashlib.md5(),
                'SHA1':   hashlib.sha1(),
                'SHA256': hashlib.sha256()}

# Color names assigned to color codes
ColorCodes = {'black': '30', 'red': '31', 'yellow': '33', 'green': '32', 'blue': '34',
                  'cyan': '36', 'magenta': '35', 'white': '37', 'gray': '90', 'reset': '0'}

######### FUNCTIONS #########


def colored(text, color):
    if not COLOR: return text
    bold = '0;'
    if BOLD: bold = '1;'
    return f'\033[{bold}{ColorCodes[str(color).lower()]}m{str(text)}\033[0m'


def DebugPrint(Category, Text, Color):
    print(colored('['+datetime.now().strftime("%H:%M:%S")+'] ', 'yellow') +
          colored('['+Category+'] ', 'magenta')+colored(Text, Color))

def fileNameFromPath(path):
    working = path.split('/')
    working = working[len(working) - 1]
    return path

def getSupplyesFilePath(args):
    DebugPrint('Args', 'Starting Parse', 'cyan')
    if len(args) < 2:
        DebugPrint('Args', 'No File Path Supplied :/', 'red')
        return ''
    path = args[1]
    DebugPrint('Args', 'Success', 'green')
    return path

def hashWithHasher(hasher, file):
    buf = open(file, 'rb').read()
    hasher.update(buf)
    return hasher.hexdigest()

def hashFileByPath(filePath):
    allHashes = {}
    for i in hashMethoods:
        DebugPrint('Hashing', f'Starting Hash {colored(i, "blue")}', 'cyan')
        result = hashWithHasher(hashMethoods[i], filePath)
        allHashes[i] = result
    DebugPrint('Hashing', 'Complete', 'green')
    return allHashes
    
def markdownTabelFromDict(data, names, filePath):
    result = f'### Hashes for `{fileNameFromPath(filePath)}`\n'
    result += f'|{names[0]}|{names[1]}|\n'
    result += f'|{"-"*len(names[0])}|{"-"*len(names[1])}|\n'
    for i in data:
        result += f'|{i}|{data[i]}|\n'
    return result[:-1]

####### MAIN FUNCTION #######


def main():
    DebugPrint('Main', 'Starting...', 'green')
    path = getSupplyesFilePath(sys.argv)
    allHashes = hashFileByPath(path)
    markdown = markdownTabelFromDict(allHashes, ['Hash', 'Value'], path)
    DebugPrint('MarkDown', 'Copy and paste to github!', 'green')
    print(markdown)
    DebugPrint('Main', 'Exiting...', 'red')


if __name__ == "__main__":
    main()