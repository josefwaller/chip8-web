import React, { useEffect, useState } from "react";

import { HexColorPicker } from "react-colorful";
import { Slider } from "rsuite";
import Popup from "reactjs-popup";

import * as styles from "../App.module.scss";

export default function Settings({ fg, bg, clockSpeed, volume, setVolume }) {
  // Store all the settings in a state so that the components are rerendered when they are changed
  // Otherwise the settings aren't kept in sync with the refs
  const [allSettings, setSettings] = useState({
    fg: fg.current,
    bg: bg.current,
    clockSpeed: clockSpeed.current,
    volume: volume,
  });
  const [modalOpen, setModalOpen] = useState(false);

  const updateColors = () => {
    document.documentElement.style.cssText = `--fg: ${fg.current}; --bg: ${bg.current}; --rs-slider-hover-bar: ${fg.current};`;
  };

  useEffect(updateColors, []);
  return (
    <>
      <button className={styles.button} onClick={() => setModalOpen(true)}>
        Settings
      </button>
      <Popup
        className="settingsPopup"
        modal
        nested
        open={modalOpen}
        closeOnDocumentClick={false}
      >
        <div className={styles.settings}>
          <a className={styles.button} onClick={() => setModalOpen(false)}>
            X
          </a>
          <span className={styles.settingsTitle}>Settings</span>
          <div className={styles.colorSettings}>
            <div>
              <div className={styles.colorTitle}>Foreground color</div>
              <HexColorPicker
                color={fg.current}
                onChange={(c) => {
                  fg.current = c;
                  setSettings({ ...allSettings, fg: c });
                  updateColors();
                }}
              />
            </div>
            <div>
              <div className={styles.colorTitle}>Background color</div>
              <HexColorPicker
                color={allSettings.bg}
                onChange={(c) => {
                  bg.current = c;
                  setSettings({ ...allSettings, bg: c });
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
                defaultValue={allSettings.clockSpeed}
                onChange={(v) => {
                  clockSpeed.current = v;
                  setSettings({ ...allSettings, clockSpeed: v });
                }}
                barClassName={styles.bar}
                handleClassName={styles.handle}
              />
            </div>
          </div>
          <div className={styles.clockSpeedSettings}>
            <div>Volume</div>
            <div>
              <Slider
                min={0}
                max={1}
                step={0.01}
                defaultValue={allSettings.volume}
                onChange={(v) => {
                  setVolume(v);
                  setSettings({ ...allSettings, volume: v });
                }}
                barClassName={styles.bar}
                handleClassName={styles.handle}
              />
            </div>
          </div>
        </div>
      </Popup>
    </>
  );
}
