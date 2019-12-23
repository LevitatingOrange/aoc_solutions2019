from sys import argv
from calendar import monthrange
from datetime import date
import requests
from rules_python.python.runfiles import runfiles

URL = "https://adventofcode.com/{year}/day/{day}/input"
USER_AGENT = {"User-Agent": "bazel"}

year = int(argv[1])

r = runfiles.Create()

with open(r.Rlocation("aoc_solutions/util/.session_cookie"), "r") as f:

    # TODO: allow multiple session cookies so one can test multiple inputs
    session_cookie = f.readlines()[0]

    for day in range(1, 26):
        if date.today() < date(year, 12, day):
            break

        print("Getting input for {}/12/{}...".format(day, year))
        url = URL.format(year=year, day=day)
        result = requests.get(url=url, cookies={"session": session_cookie}, headers=USER_AGENT)
        if result.status_code != 200:
            print("Could not get puzzle input with url \"{}\"".format(url))
            raise IOError
        with open(f'input_{day:02}', "w") as of:
            of.write(result.text)
        
