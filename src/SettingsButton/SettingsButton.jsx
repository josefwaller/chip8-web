import React, { useEffect } from "react";

import { HexColorPicker } from "react-colorful";
import { Slider } from "rsuite";
import Popup from "reactjs-popup";

import * as styles from "../App.module.scss";

export default function Settings({ fg, bg, clockSpeed }) {
  const updateColors = () => {
    document.documentElement.style.cssText = `--fg: ${fg.current}; --bg: ${bg.current}; --rs-slider-hover-bar: ${fg.current};`;
  };

  // Initialise colours
  useEffect(updateColors, []);
  return (
    <Popup
      trigger={<button className={styles.button}>Settings</button>}
      className="settingsPopup"
      modal
      nested
    >
      <div className={styles.settings}>
        <span className={styles.settingsTitle}>Settings</span>
        <div className={styles.colorSettings}>
          <div>
            Foreground color
            <HexColorPicker
              color={fg.current}
              onChange={(c) => {
                fg.current = c;
                updateColors();
              }}
            />
          </div>
          <div>
            Background colour
            <HexColorPicker
              color={bg.current}
              onChange={(c) => {
                bg.current = c;
                updateColors();
              }}
            />
          </div>
        </div>
        <div className={styles.clockSpeedSettings}>
          <div>Clock Speed</div>
          <div>
            <Slider
              min={10}
              max={2000}
              defaultValue={clockSpeed.current}
              onChange={(v) => (clockSpeed.current = v)}
              barClassName={styles.bar}
              handleClassName={styles.handle}
            />
          </div>
        </div>
      </div>
    </Popup>
  );
}
