# rust_relm_and_camera_practice

# Requirement
[Rust](https://www.rust-lang.org)

Tested in 1.22.1.

# Setup
```
sudo apt install libv4l-dev
sudo apt install libgtk-3-dev
```

# Run
```
cargo run
```

# License
MIT

# References
- [rscamとrelmを利用して、カメラの画像を表示するアプリをrustで作る方法](http://asukiaaa.blogspot.com/2018/01/rscamrelmrust.html)
- [Are there any video capture libraries out there?](https://users.rust-lang.org/t/are-there-any-video-capture-libraries-out-there/11241)
- [Struct gtk::Image](http://gtk-rs.org/docs/gtk/struct.Image.html)
- [How to draw in-memory images to a window?](https://github.com/gtk-rs/gtk/issues/28)
- [Trait gtk::ImageExt](http://gtk-rs.org/docs/gtk/trait.ImageExt.html)
- [GTKとRustでLinuxデスクトップアプリ入門](https://qiita.com/koji_mats/items/62e85a87cc580e225796)
- [2.13. Reserved Format Identifiers](https://linuxtv.org/downloads/v4l-dvb-apis/uapi/v4l/pixfmt-reserved.html?highlight=mjpg)
- [Gdk pixbuf load image from memory](https://stackoverflow.com/questions/14121166/gdk-pixbuf-load-image-from-memory)
- [Struct gdk_pixbuf::PixbufLoader](http://gtk-rs.org/docs/gdk_pixbuf/struct.PixbufLoader.html#method.loader_write)
- [How do I update/redraw a GTK Widget (GTKLabel) internally without a key press event using python?
](https://stackoverflow.com/questions/8381631/how-do-i-update-redraw-a-gtk-widget-gtklabel-internally-without-a-key-press-ev)
- [fn connect_exec_ignore_err<CALLBACK, STREAM, TOSTREAM>](https://docs.rs/relm/0.11.0/relm/struct.Relm.html#method.connect_exec_ignore_err)
- [relm/examples/async/src/main.rs](https://github.com/antoyo/relm/blob/74bb6e35641a14edeca286506f17fea1ce9ebadb/examples/async/src/main.rs)
