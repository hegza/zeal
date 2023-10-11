# Feature backlog

In priority order. Updated if I feel like it.

| Feature | UID | Status |
| - | - | - |
| Viewport can be moved with arrows | VIEW1 | WIP |
| Viewport can be moved with mouse | VIEW2 | DESIGN |

Status list:

- DONE: if it doesn't work, move to broken
- BROKEN: used to work but now broken
- WIP: under construction
- DESIGN: design started
- empty: not started

## Detailed descriptions

### VIEW2

Viewport can be moved with mouse, unless another object captures the input.

#### Bugs

- Drag movement is not exact. Delta should be relative to real pixels.
- Drags should only be accepted if they start within viewport
