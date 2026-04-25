# Configuring st
`st` uses a toml file (called `st.toml`) for configuring it. Currently, there aren't really that many options. But here is a short list:

## ***active***

The `active` parameter allows you to set labels that always get used in `st list` operations. Using this parameter you can simulate "currently active sprint."
```toml
# st.toml
active = "SPRINT:1"
```
If you set active to, say, `"SPRINT:1,ASSIGNE:user1"` and run:
```sh
st list
```
It will have the same effect as running:
```sh
st list -l'SPRINT:1,ASSIGNEE:user1'
```
You can still add other labels to your command so:
```sh
st list -l'COMPONENT:st'
```
Will NOT overwrite the **active** parameter from `st.toml`. Instead it will expand it. So in reality your command will look like:
```sh
st list -l'SPRINT:1,ASSIGNEE:user1,COMPONENT:st'
```
The **active** parameter can be overwritten using the `-b` or `--bypass-config` flags.

## ***apply***

Here, by "applying" we mean what `st apply` does (i.e. Transforming `{topic}/STATE.md` into proper task attributes). This parameter has 4 possible states:
```toml
apply = "enabled" # default
apply = "always" # applies the STATE.md every time a `st` command is run
apply = "delete" # also deletes the STATE.md each time `st apply` is run (by default, the STATE.md file remains on disk)
apply = "never"  # disables the apply feature all together...
apply = "always&delete" # same as "always" but it also deletes the STATE.md files...
```

