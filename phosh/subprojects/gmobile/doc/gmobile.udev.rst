.. _gmobile.udev(5):

============
gmobile udev
============

--------------------------------
Device configuration for gmobile
--------------------------------

DESCRIPTION
-----------

``gmobile`` allows one to configure certain aspects through
`udev's hwdb <https://www.freedesktop.org/software/systemd/man/hwdb.html>`_

The following properties are supported:

WAKEUP KEYS
-----------

An input device's wakeup keys specify which keys unblank the screen of
an idle device. By default all keys are wakeup keys. The default can
be changed on a per keyboard level via

::

  GM_WAKEUP_KEY_DEFAULT=0

The behaviour of individual keys can be changed by giving their keycodes e.g.

::

  GM_WAKEUP_KEY_<keycode>=[0|1]

The keycode is the linux event code.

Note that gmobile merely provides that information. The Wayland compositor is
responsible for applying it.

For details on how to add these properties to hwdb see below.

CONFIGURING HWDB
----------------

The hwdb contains a set of match rules that assign udev properties
that become available when the device is connected. This section only
describes the hwdb in relation to gmobile, it is not full
documentation on how the hwdb works. For that please see the
``hwdb(7)`` man page.

gmobile's use of the hwdb is limited to properties systemd and custom
rules files (where available) provide.

.. _hwdb_querying:

.................
Querying the hwdb
.................

gmobile currently only uses device nodes in the form of ``/dev/input/eventX`` where X
is the number of the specific device. Running ``libinput debug-events`` lists
all devices currently available to libinput and their event node name: ::

    $> sudo libinput debug-events
    -event0   DEVICE_ADDED            gpio-keys                         seat0 default group1  cap:k
    -event2   DEVICE_ADDED            30370000.snvs:snvs-powerkey       seat0 default group2  cap:k
    -event3   DEVICE_ADDED            generic ft5x06 (f0)               seat0 default group3  cap:t ntouches 10 calib
    ...

Note the event node name for your device and translate it into a syspath in
the form of ``/sys/class/input/eventX``. This path can be supplied to ``udevadm
info`` ::

    $> udevadm info -p /sys/class/input/event0/
    P: /devices/platform/gpio-keys/input/input0/event0
    M: event0
    R: 0
    U: input
    D: c 13:64
    N: input/event0
    L: 0
    S: input/by-path/platform-gpio-keys-event
    E: DEVPATH=/devices/platform/gpio-keys/input/input0/event0
    E: SUBSYSTEM=input
    E: DEVNAME=/dev/input/event0
    E: MAJOR=13
    E: MINOR=64
    E: USEC_INITIALIZED=5327886
    E: ID_INPUT=1
    E: ID_INPUT_KEY=1
    E: ID_PATH=platform-gpio-keys
    E: ID_PATH_TAG=platform-gpio-keys
    E: GM_WAKEUP_KEY_114=0
    E: GM_WAKEUP_KEY_115=0
    …

Lines starting with ``E:`` are udev properties available to applications. Properties
added by gmobile all have a `GM_` prefix. They are only present if a hwdb entry
matches.

.. _hwdb_reloading:

..................
Reloading the hwdb
..................

The actual hwdb is stored in a binary file on-disk and must be updated
manually whenever a ``.hwdb`` file changes. This is required both when a user
manually edits the ``.hwdb`` file or when gmobile ships an updated set of entries.

To update the binary file on-disk, run: ::

    sudo systemd-hwdb update

Then, to trigger a reload of all properties on your device, run: ::

    sudo udevadm trigger /sys/class/input/eventX

Then check with ``udevadm info`` whether the properties were updated, see
:ref:`hwdb_querying`. If a new property does not appear on the device, use ``udevadm
test`` to check for error messages by udev and the hwdb (e.g. syntax errors
in the udev rules files). ::

    sudo udevadm test /sys/class/input/eventX

.. _hwdb_modifying:

..............................................................................
Modifying the hwdb
..............................................................................

This section applies to users that need to add, change, or remove a hwdb
entry for their device. Note that **the hwdb is not part of the public API
and may change at any time**. Once a device has been made to work, the
change must be submitted to the
`gmobile repository  <https://gitlab.gnome.org/World/Phosh/gmobile>`_.

hwdb entries are only applied if a udev rules calls out to the hwdb with the
right match format. gmobile ships with a set of rules to query the hwdb,
the different rules are reflected by their prefix. Again, **this is not part
of the public API**. gmobile's matches are
composed of a literal "gmobile", then either the device name (prefixed
with `name:`) followed by the machine's first device tree comptible
(prefixed with `dt:`) or dmi modalias. For example:

::

    gmobile:name:gpio-keys:dt:purism,librem5*

The device name is available in the device's `/sys/class/input/eventX/device/name`
while the device tree compatible is available in `/sys/firmware/devicetree/base/compatible`.

The hwdb match string is the first portion of the hwdb entry. The second
portion is the property to set. Each hwdb entry may match on multiple
devices and may apply multiple properties. For example:

::

    gmobile:name:gpio-keys:dt:purism,librem5*
      GM_WAKEUP_KEY_114=0
      GM_WAKEUP_KEY_115=0

In the example above the matching gpio-keys device will have both
properties applied.

The hwdb does not allow removing properties. Where a property must be unset,
it should be set to 0.

For testing any user-specific hwdb entries should be placed in a file
`/etc/udev/hwdb.d/99-gmobile.hwdb` but please make sure to submit them upstream
as the hwdb format might change without notice.

See also
--------

``hwdb(7)`` ``systemd-hwdb(8)`` ``phoc(1)``
