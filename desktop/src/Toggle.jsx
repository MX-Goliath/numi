import React, { useEffect, useState } from "react";
import "./Toggle.css";

export function Toggle({ initPosition, onChange, onMouseEnter, onMouseLeave }) {
  const [isOn, setIsOn] = useState(Boolean(initPosition));
  useEffect(() => setIsOn(Boolean(initPosition)), [initPosition]);
  const toggle = () => { const next = !isOn; setIsOn(next); onChange(next); };
  return <button type="button" className={`toggle ${isOn ? "on" : ""}`} aria-label="High Precision" aria-pressed={isOn} onClick={toggle} onMouseEnter={onMouseEnter} onMouseLeave={onMouseLeave}><span /></button>;
}
