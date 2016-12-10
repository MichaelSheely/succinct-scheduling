# succinct-scheduling
A DSL which allows users to describe their schedules succinctly and precisely.

By Michael Sheely

## How To Use Succinct Scheduler

In this git repository, you will find two files within the `executables`
directory.

The leader will need the `leader` file.  They should ask everyone involved
in the event to either send them their schedule in a text file formatted
according to the schema described below or to send them an exported calendar
(it should have a `.ics` extension).

The users who opt to write up a text file can ensure they have the correct
format by running `parse_file` in the same directory as the text file
containing their schedule and either type the name when prompted or give
it on the command line as an argument.

If they correctly formatted their text file, the `parse_file` program should
print out the schedule of the user, which they can verify was entered
without error.

Once all users have verified their own schedule or exported a `.ics` calendar
document, they can send these documents to the organized.  This leader will
put all of the text files, the calendar documents, and the `leader` executable
itself in the same directory somewhere on their machine and then run `./leader`
from that directory.

They should follow the prompts and then the language should find the times when
the most individuals are free within the leader's specified time range.

## Example Program

Here we provide some example programs designed to demonstrate the possible
options for syntax of the language:

A file `schedule.txt` shown below:
```
Monday, Tuesday:
  free 10am, 1pm
  free(2pm, 3pm)
Monday:
  free 1pm - 5pm
```

Another schedule file, `schedule2.txt` shown below:
```
Mon, Tues:
  free  1pm - 2pm
  free 4pm, 5pm except 12/5/16
Wednesday:
  free 11am - 2pm
```
