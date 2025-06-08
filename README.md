### 📄 `README.md`

````markdown
# 🌤️ Rust Weather App

A small command-line application written in Rust that fetches the current weather for a specified city and prints it in a compact format — e.g., `25°C ☀️`.

Perfect for use with `nwg-panel` under Wayland (e.g., `sway` or `hyprland`) to display the weather next to your clock.

---

## 📦 Dependencies

- [Rust](https://www.rust-lang.org/)
- [reqwest](https://docs.rs/reqwest)
- [serde](https://serde.rs/)
- [anyhow](https://docs.rs/anyhow)
- [wttr.in](https://wttr.in/) — external weather API

---

## ⚙️ Configuration

The app looks for a `weather_config.toml` file in the following locations (in order of priority):

1. In the same directory as the executable
2. In the current working directory
3. In `~/.config/weather_plugin/config.toml`

If the config file is missing, a default one will be automatically generated:

```toml
city = "London"
units = "metric"
refresh_interval = 300
language = "en"
````

---

## 🔧 Using with `nwg-panel`

To display the weather in your `nwg-panel`:

1. Open the graphical config editor:

   ```bash
   nwg-panel-config
   ```

2. Add an `executor` module named `executor-weather` to the **center section** of your top panel.

3. Set the command to your weather binary:

   ```
   /home/USERNAME/rust_weather_app/weather
   ```

4. Set the refresh interval (e.g., `300` seconds).

5. Save and restart the panel:

   ```bash
   pkill nwg-panel && nwg-panel &
   ```

---

## 📦 Build

```bash
git clone https://github.com/yourname/rust_weather_app
cd rust_weather_app
cargo build --release
```

After building, the binary will be located at:

```
target/release/weather
```

Copy it to your local bin if needed:

```bash
cp target/release/weather ~/.local/bin/weather
```

---

## 💡 Example Output

```
21°C ⛅️
```

---

## 🔒 Notes

* The `wttr.in` API does not require an API key, but be mindful of usage.
* Weather condition codes are mapped to Unicode emojis (e.g., `113` → ☀️).

---

## 📁 Project Structure

```
rust_weather_app/
├── src/
│   ├── main.rs        # main application code
│   └── config.rs      # configuration handling
├── weather_config.toml (optional)
├── Cargo.toml
└── README.md
```

---

## 🛠 TODO

* [ ] Add somethink ...

---

## 📝 License

MIT



