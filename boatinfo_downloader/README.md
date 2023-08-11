# boatinfo_downloader

Scrapes boat data from [boatinfoworld.com](https://www.boatinfoworld.com) and outputs it to a CSV file.
It does this in two _phases_: the first phase gets the number of boats in each US state, and the second phase goes on chunks of 100 boats (simultaneously) and gets the data for each boat.
This data includes each boat's:

- Registration state
- Name
- Manufacturer
- Manufacture Year
- Type
- Weight
- Length
- Id
