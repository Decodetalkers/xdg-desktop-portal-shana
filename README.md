# xdg-desktop-portal-shana

[![Packaging status](https://repology.org/badge/vertical-allrepos/xdg-desktop-portal-shana.svg)](https://repology.org/project/xdg-desktop-portal-shana/versions)


## How it works
it just redirect other portal to it, `the portal of portal`, maybe?

now it just use

* Gnome
* Kde
* Gtk
* Lxqt

## How to use

create ~/.config/xdg-desktop-portal-shana/config.toml

write like this

```toml
open_file = "Kde"
save_file = "Gnome"

[tips]
open_file_when_folder = "Lxqt"
```

The keyword you can use include

`Gnome`
`Kde`
`Gtk`
`Lxqt`

## Show

![select file](./show/choosefile.png)

![save file](./show/savefile.png)
