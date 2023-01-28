# i3-MultiMonitor
A simple Rust application to adjust how the i3 window manager handles workspaces on multiple monitors

#### About
i3-MultiMonitor implements the IPC api of the i3 window manager to intercept workspace related keybinds and adjusts them to be monitor specific. More specifically, if your keybind is workspace specific, it will intercept the keybind command and replace the specified workspace with the appropriate workspace for the currently focused monitor.

For example, if you have workspace 1 on monitor 1, workspace 2 on monitor 2, and request to switch to workspace 1 on monitor 2, this application will adjust your keybind command to switch to workspace 11 on monitor 2. This is an adaptation of the normal i3 behavior, which would have moved focus to monitor 1 and switched to workspace 1 if it was not currently focused.

The end result is an IPC implementation of monitor specific workspaces in i3. Each monitor has it's own set of workspaces, and referencing a workspace on one monitor will never activate a workspace on another monitor. The only i3 config change required is to set your existing workspace keybinds to `nop` - 
```
# switch to workspace
bindsym $mod+1 nop
bindsym $mod+2 nop
...
bindsym $mod+8 nop
bindsym $mod+9 nop

# move window to workspace
bindsym $mod+Shift+1 nop
bindsym $mod+Shift+2 nop
...
bindsym $mod+Shift+8 nop
bindsym $mod+Shift+9 nop
```

A full set of workspaces using i3-MultiMonitor would look something like this - 
```
Monitor 1 - 1, 2, 3, 4, 5, 6, 7, 8, 9
Monitor 2 - 11,12,13,14,15,16,17,18,19
```

It is helpful, but not necessary to configure i3 to only allow certain workspaces on certain monitors.
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

For a fully seamless experience, it is possible to adjust the workspace widget of your bar application to display truncated values for the additional workspaces. Polybar example - 
```
[module/i3]
type = internal/i3

; Workspaces for monitor 1
ws-icon-0 = 1;1
ws-icon-1 = 2;2
...
ws-icon-8 = 8;8
ws-icon-9 = 9;9

; Workspaces for monitor 2
ws-icon-10 = 11;1
ws-icon-11 = 12;2
...
ws-icon-17 = 18;8
ws-icon-18 = 19;9
```

This would allow for a full set of workspaces to look like this - 
```
Monitor 1 - 1, 2, 3, 4, 5, 6, 7, 8, 9
Monitor 2 - 1, 2, 3, 4, 5, 6, 7, 8, 9
```
