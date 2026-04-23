# st (Shell Tracker)
`st` is a issue / task tracker designed for small teams (and/or personal use). It can be used alongside a source control management tool such as `git` or `jj`, or (although it is not yet recommended to do so) it can also be used as standalone, since it provides a bare bones source control of its own. (checkout `st-pull`, `st-init` or `st-push`)

## Basic Examples
Listing tasks from "Testing" topic that are either in sprint one or are assigned to alex:
```sh
$ st list -T Testing -l 'SPRINT:1,ASSIGNEE:alex' -vn
Make.CI.And.Unit.Tests
  Labels:
    SPRINT:1
    ASSIGNEE:alex
  Description:
    We will need to have a way to somewhat test the project through a UT CI.

  Notes:
    Note to self. Dont forget to use
    cargo clippy pedantic.
Whatever
  Labels:
    ASSIGNEE:alex
  Notes:
    ## adding labels, and no longer ignoring st dir (af8f61a) by Ana Alexandru-Gabriel<alexandru.ana@nokia.com> on Wed, 22 Apr 2026 17:55:12 +0300
    I added labels...
```
Adding note with pre-appended git commit:
```sh
# this should pre-append the git commit details and then open notes in your editor
$ st add -T Testing -t Make.CI.And.Unit.Tests -nc
$ st list -T Testing -l 'SPRINT:1' -vn
Make.CI.And.Unit.Tests
  Labels:
    SPRINT:1
    ASSIGNEE:alex
  Description:
    We will need to have a way to somewhat test the project through a UT CI.

  Notes:
    Note to self. Dont forget to use
    cargo clippy pedantic.
    ## adding labels, and no longer ignoring st dir (af8f61a) by Ana Alexandru-Gabriel<alexandru.ana@nokia.com> on Wed, 22 Apr 2026 17:55:12 +0300
    The line above this one was added automatically because of the `-c`
```
## Installing
Clone the repository:
```sh
git clone https://github.com/kickhead13/st
```
And install it:
```sh
cd st
cargo install --path .
export PATH="$PATH:$HOME/.cargo/bin" # you should probably add this to your .bashrc if you'd like st to always be available
```
## Usage
Initializing a `st` repository:
```sh
# this creates a bare bones .st directory in your current workding directory
st init 
```
Create a topic:
```sh
st add -T topic1
```
Add a task to that topic:
```sh
st add -T topic1 -a task1
```
Edit the description of the tag:
```sh
st add -T topic1 -t task1 -d
```
Edit the notes of the tag:
```sh
st add -T topic1 -t task1 -n
```
Edit the labels of the tag:
```sh
st add -T topic1 -t task1 -l
```
Listing tasks:
```sh
st list -T topic1
```
For more information use the help messages of subcommands.

## Bash Completion
To have a better flow using `st` please also source the `bash_completion.sh` available in the repository:
```sh
source  bash_completion.sh
# now you should have auto completion in bash shells
# unfortunately... support for other shells such as zsh is not yet available.
```
