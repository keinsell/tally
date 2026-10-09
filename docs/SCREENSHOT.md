The app listing (icon, description, screenshots) comes from
`data/dev.keinsell.Counter.metainfo.xml.in`. Screenshots live in
`screenshots/` and are referenced by public URL, so they must be pushed to
GitHub before stores can fetch them.

To re-capture `screenshots/main.png` (needs Xvfb and ImageMagick):

```sh
Xvfb :99 -screen 0 512x384x24 &
env -u WAYLAND_DISPLAY WAYLAND_DISPLAY= GDK_BACKEND=x11 DISPLAY=:99 ./target/release/counter &
sleep 4
# drive the app to a lively state over D-Bus, then shoot
for i in 1 2 3; do
  gdbus call --session --dest dev.keinsell.Counter \
    --object-path /dev/keinsell/Counter \
    --method org.gtk.Actions.Activate "increment" "[]" "{}"
  sleep 1
done
DISPLAY=:99 import -window root /tmp/shot.png
convert /tmp/shot.png -crop 400x300+0+0 +repage \
  -bordercolor black -border 28x28 screenshots/main.png
```

When cutting a release, also add a `<release>` entry with the new version and
date to the metainfo, and refresh the screenshots if the UI changed.
