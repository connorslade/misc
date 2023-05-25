# Youtube Stats

I wanted to know how much of my life ive spent on YouTube.
So I downloaded my data from Google, which included my youtube history, the used this program to query the length of each video using the YouTube API then output the data to a CSV file for manual review.

Because each API application is only allowed 10k requests per day, and I needed to query ~60k videos, this program can take in multiple API keys that it will switch between.
