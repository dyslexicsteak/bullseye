# bullseye
A tool to clean up your target directories as they usually take up a significant amount of disk space.

## Usage
This tool only takes 2 arguments:
  - Directory: The directory whose children are going to be cleaned.
  - (Optional) Duration: The minimum time since the last modifcation to the directory for it to be considered dormant. This defaults to 1 week.

## Duration Format
The duration format is that of the `duration-str` crate.

| unit | Description | unit list option(one of)                                                                         | example |
| ---- | ----------- | ------------------------------------------------------------------------------------------------ | ------- |
| y    | Year        | ["y" , "year" , "Y" , "YEAR" , "Year"]                                                           | 1y      |
| mon  | Month       | ["mon" , "MON" , "Month" , "month" , "MONTH"]                                                      | 1mon    |
| w    | Week        | ["w" , "W" , "Week" ,"WEEK" , "week"]                                                              | 1w      |
| d    | Day         | ["d" , "D" , "Day" , "DAY" , "day"]                                                                | 1d      |
| h    | Hour        | ["h" , "H" , "Hour" , "HOUR" , "hour"]                                                             | 1h      |
| m    | Minute      | ["m" , "M" , "Minute" , "MINUTE" , "minute" , "min" , "MIN"]                                       | 1m      |
| s    | Second      | ["s" , "S" , "Second" , "SECOND" , "second" , "sec" , "SEC"]                                       | 1s      |
| ms   | Millisecond | ["ms" , "MS" , "Millisecond" , "MilliSecond" , "MILLISECOND" , "millisecond" , "mSEC"]             | 1ms     |
| µs   | Microsecond | ["µs" , "µS" , "µsecond" , "Microsecond" , "MicroSecond" , "MICROSECOND" , "microsecond" , "µSEC"] | 1µs     |
| ns   | Nanosecond  | ["ns" , "NS" , "Nanosecond" , "NanoSecond" , "NANOSECOND" , "nanosecond" , "nSEC"]                 | 1ns     |
