The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

The sections should follow the order `Added`, `Changed`, `Deprecated`,
`Removed`, and `Fixed`.

Platform specific changed should be added to the end of the section and grouped
by platform name. Common API additions should have `, implemented` at the end
for platforms where the API was initially implemented. See the following example
on how to add them:

```md
### Added

- Add `Window::turbo()`, implemented on X11, Wayland, and Web.
- On X11, add `Window::some_rare_api`.
- On X11, add `Window::even_more_rare_api`.
- On Wayland, add `Window::common_api`.
- On Windows, add `Window::some_rare_api`.
```

When the change requires non-trivial amount of work for users to comply
with it, the migration guide should be added below the entry, like:

```md
- Deprecate `Window` creation outside of `EventLoop::run`

  This was done to simply migration in the future. Consider the
  following code:

  // Code snippet.

  To migrate it we should do X, Y, and then Z, for example:

  // Code snippet.

```

The migration guide could reference other migration examples in the current
changelog entry.

## Unreleased

### Fixed

- On Windows, release the cursor clip when the window loses focus and reapply it on focus gain.
- On Windows, preserve requested decorations and normal activation behavior for ordinary owned windows, such as dialogs.
- On Windows, fix the IME preedit cursor appearing before the composition with IMEs such as the Microsoft Korean IME that report neither a cursor position (`GCS_CURSORPOS`) nor a target clause; it is now placed at the end.
- On Windows, emit `WindowEvent::SurfaceResized` from `WM_NCCALCSIZE` instead of `WM_SIZE`. The compositor shows a frame presented from this event together with the new window rectangle, which allows flicker-free live resizing; frames presented later are shown stretched for a moment.
- On Windows, keep a transparent window transparent after leaving fullscreen. The windowed position and size are now restored in a later message.
- On macOS, only close windows created by Winit when the event loop exits. Closing every window of the application also closed the window behind an `NSStatusItem`, which left the menu bar item unable to receive clicks after `run_app_on_demand` returned.
- On Windows, fix a deadlock when IME is disabled during an active composition. The input
  method sends `WM_IME_ENDCOMPOSITION` synchronously, and its handler waited on the window
  state lock that `request_ime_update` was still holding.
