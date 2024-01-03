# i3-MultiMonitor
A simple Rust application to adjust how the i3 window manager handles workspaces on multiple monitors

## About
i3-MultiMonitor implements the IPC api of the i3 window manager to intercept workspace related keybinds and adjust them to be monitor dependent.

More specifically, it will intercept the workspace keybind sent by the user and replace the specified workspace with the i3 recognized workspace for the currently focused monitor.

This enables the user to have one set of keybinds to interact with what is effectively an independent set of workspaces per monitor 

## Execution Flow
Lets assume that you have workspace 1 and 2 on monitor 1, workspace 6 and 7 on monitor 2, with monitor 2 focused, currently displaying workspace 7.

You then press a keybind that requests to switch to workspace 1.

This application will:
 - Intercept the i3 command sent by your keybind -> "workspace 1" in this case
 - Find the currently focused monitor -> "monitor 2"
 - Calculate the index of the workspace that would be *considered* workspace 1 on that monitor -> "workspace 6"
 - Alter the i3 command to switch to that workspace instead -> "i3-msg workspace 6"
 - Execute the command
 
This is an adaptation of the normal i3 behavior, which would have moved focus to monitor 1, then switched to workspace 1 if it was not already being displayed.

The end result is an IPC implementation of monitor specific workspaces in i3.

## Assumptions
i3-MultiMonitor makes a few assumptions:

 - Your i3 config workspace keybinds are set to `nop`
 - There will only be 5 workspaces per monitor
 - Your switch to workspace keybind *does not* include shift
 - Your move window to workspace keybind *does* include shift
 - When moving a window to a workspace, you do not intend to also switch to that workspace

## Configuration Example
~/.config/i3/config
```
# switch to workspace
bindsym $mod+1 nop
bindsym $mod+2 nop

# move window to workspace
bindsym $mod+Shift+1 nop
bindsym $mod+Shift+2 nop
...
bindsym $mod+Shift+8 nop
bindsym $mod+Shift+9 nop
```

### Tips & Tricks
#### Assign Workspaces To Monitors
It may be helpful to configure i3 to assign workspaces to monitors, to prevent workspaces moving between monitors accidentally
```
# Monitor 1 workspaces
workspace 1 output primary
workspace 2 output primary
...
workspace 8 output primary
workspace 9 output primary

# Monitor 2 workspaces
workspace 11 output nonprimary
workspace 12 output nonprimary
...
workspace 18 output nonprimary
workspace 19 output nonprimary
```
