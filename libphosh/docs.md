<!-- file * -->
<!-- static APP_UNKNOWN_ICON -->
Icon name to use for apps we can't identify or whose icon is invalid
<!-- impl LockscreenBuilder::fn page -->
The currently active carousel page
<!-- impl LockscreenBuilder::fn accept_focus -->
Whether the window should receive the input focus.
<!-- impl LockscreenBuilder::fn application -->
The #GtkApplication associated with the window.

The application will be kept alive for at least as long as it
has any windows associated with it (see g_application_hold()
for a way to keep it alive without windows).

Normally, the connection between the application and the window
will remain until the window is destroyed, but you can explicitly
remove it by setting the :application property to [`None`].
<!-- impl LockscreenBuilder::fn attached_to -->
The widget to which this window is attached.
See gtk_window_set_attached_to().

Examples of places where specifying this relation is useful are
for instance a #GtkMenu created by a #GtkComboBox, a completion
popup window created by #GtkEntry or a typeahead search entry
created by #GtkTreeView.
<!-- impl LockscreenBuilder::fn decorated -->
Whether the window should be decorated by the window manager.
<!-- impl LockscreenBuilder::fn deletable -->
Whether the window frame should have a close button.
<!-- impl LockscreenBuilder::fn focus_on_map -->
Whether the window should receive the input focus when mapped.
<!-- impl LockscreenBuilder::fn focus_visible -->
Whether 'focus rectangles' are currently visible in this window.

This property is maintained by GTK+ based on user input
and should not be set by applications.
<!-- impl LockscreenBuilder::fn gravity -->
The window gravity of the window. See gtk_window_move() and #GdkGravity for
more details about window gravity.
<!-- impl LockscreenBuilder::fn has_resize_grip -->
Whether the window has a corner resize grip.

Note that the resize grip is only shown if the window is
actually resizable and not maximized. Use
#GtkWindow:resize-grip-visible to find out if the resize
grip is currently shown.
Resize grips have been removed.
<!-- impl LockscreenBuilder::fn hide_titlebar_when_maximized -->
Whether the titlebar should be hidden during maximization.
<!-- impl LockscreenBuilder::fn icon_name -->
The :icon-name property specifies the name of the themed icon to
use as the window icon. See #GtkIconTheme for more details.
<!-- impl LockscreenBuilder::fn mnemonics_visible -->
Whether mnemonics are currently visible in this window.

This property is maintained by GTK+ based on user input,
and should not be set by applications.
<!-- impl LockscreenBuilder::fn startup_id -->
The :startup-id is a write-only property for setting window's
startup notification identifier. See gtk_window_set_startup_id()
for more details.
<!-- impl LockscreenBuilder::fn transient_for -->
The transient parent of the window. See gtk_window_set_transient_for() for
more details about transient windows.
<!-- impl LockscreenBuilder::fn halign -->
How to distribute horizontal space if widget gets extra space, see #GtkAlign
<!-- impl LockscreenBuilder::fn style -->
The style of the widget, which contains information about how it will look (colors, etc).
Use #GtkStyleContext instead
<!-- impl LockscreenBuilder::fn valign -->
How to distribute vertical space if widget gets extra space, see #GtkAlign
<!-- impl ShellBuilder::fn builtin_monitor -->
The built in monitor. This is a hardware property and hence can
only be read. It can be [`None`] when not present or disabled.
<!-- impl ShellBuilder::fn primary_monitor -->
The primary monitor that has the panels, lock screen etc.
<!-- trait ShellExt::fn activate_action -->
Activates the given action. If the action is not found [`false`] is returned and a
warning is logged.
## `action`
The action name
## `parameter`
The action's parameters

# Returns

[`true`] if the action was found
<!-- trait ShellExt::fn app_launch_context -->

# Returns

an app launch context for the primary display
<!-- trait ShellExt::fn app_tracker -->
Get the app tracker

# Returns

The app tracker
<!-- trait ShellExt::fn background_manager -->
Get the background manager

# Returns

The background manager
<!-- trait ShellExt::fn builtin_monitor -->

# Returns

the built in monitor or [`None`] if there is no built in monitor
<!-- trait ShellExt::fn calls_manager -->
Get the calls manager

# Returns

The calls manager
<!-- trait ShellExt::fn docked_manager -->
Get the docked manager

# Returns

The docked manager
<!-- trait ShellExt::fn emergency_calls_manager -->
Get the emergency calls manager

# Returns

The emergency calls manager
<!-- trait ShellExt::fn feedback_manager -->
Get the feedback manager

# Returns

The feedback manager
<!-- trait ShellExt::fn gtk_mount_manager -->
Get the GTK mount manager

# Returns

The GTK mount manager
<!-- trait ShellExt::fn hks_manager -->
Get the hardware killswitch manager

# Returns

The hardware killswitch manager
<!-- trait ShellExt::fn launcher_entry_manager -->
Get the launcher entry manager

# Returns

The launcher entry manager
<!-- trait ShellExt::fn layout_manager -->
Get the layout manager

# Returns

The layout manager
<!-- trait ShellExt::fn location_manager -->
Get the location manager

# Returns

The location manager
<!-- trait ShellExt::fn lockscreen_manager -->
Get the lockscreen manager

# Returns

The lockscreen manager
<!-- trait ShellExt::fn mode_manager -->
Get the mode manager

# Returns

The mode manager
<!-- trait ShellExt::fn monitor_manager -->
Get the monitor manager

# Returns

The monitor manager
<!-- trait ShellExt::fn osk_manager -->
Get the onscreen keyboard manager

# Returns

The onscreen keyboard manager
<!-- trait ShellExt::fn primary_monitor -->

# Returns

the primary monitor or [`None`] if there currently are no outputs
<!-- trait ShellExt::fn rotation_manager -->
Get the rotation manager

# Returns

The rotation manager
<!-- trait ShellExt::fn screen_saver_manager -->
Get the screensaver manager

# Returns

The screensaver manager
<!-- trait ShellExt::fn screenshot_manager -->
Get the screenshot manager

# Returns

The screenshot manager
<!-- trait ShellExt::fn session_manager -->
Get the session manager

# Returns

The session manager
<!-- trait ShellExt::fn state -->

# Returns

The current #PhoshShellStateFlags
<!-- trait ShellExt::fn toplevel_manager -->
Get the toplevel manager

# Returns

The toplevel manager
<!-- trait ShellExt::fn torch_manager -->
Get the torch manager

# Returns

The torch manager
<!-- trait ShellExt::fn vpn_manager -->
Get the VPN manager

# Returns

The VPN manager
<!-- trait ShellExt::fn wifi_manager -->
Get the Wifi manager

# Returns

The Wifi manager
<!-- trait ShellExt::fn wwan -->
Get the WWAN manager

# Returns

The WWAN manager
<!-- trait ShellExt::fn set_state -->
Set the shells state.
## `state`
The #PhoshShellStateFlags to set
## `enabled`
[`true`] to set a shell state, [`false`] to reset
<!-- trait ShellExt::fn docked -->
Whether the device is currently docked. This mirrors the property
from #PhoshDockedManager for easier access.
<!-- trait ShellExt::fn locked -->
Whether the screen is currently locked. This mirrors the property
from #PhoshLockscreenManager for easier access.
<!-- trait WallClockImpl::fn clock -->
Gets the current clock string, if time_only is true this will be just the
current time, otherwise the date + time.
## `time_only`
whether to return full clock string or just the time

# Returns

the clock time string
