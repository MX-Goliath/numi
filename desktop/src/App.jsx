import { useState, useEffect, useRef, useCallback } from "react";
import { MultiEditor } from "./MultiEditor.jsx";
import "./App.css";
import { AppCalc } from "./app_calc.js";
import { Toggle } from "./Toggle.jsx";

let invoke;

if (true) {
  async function real_invoker(method, params) {
    if (method == "calc_json") {
      let res = await window.electronAPI.numiModelCalc(params.text);
      return res;
    }
  }
  invoke = real_invoker;
} else {
  async function test_invoker(method, params) {
    if (method == "calc_json") {
      let input = JSON.parse(params.text);
      let lines = input.input;
      let result = [];
      for (const el of lines) {
        result.push({
          value: el.length,
        });
      }
      return JSON.stringify({ result: result });
    }
  }
  invoke = test_invoker;

  console.log("Test invoker");
}

const calc = new AppCalc(invoke);

function App() {
  const [text, setText] = useState("");
  const [values, setValues] = useState([]);
  const [version, setVersion] = useState("0.0");
  // The native model creates its offline calculator before refreshing rates.
  // Starting ready keeps the first expression from depending on an IPC event
  // that may arrive before React has mounted.
  const [engineReady, setEngineReady] = useState(true);
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [updateDisabled, setUpdateDisabled] = useState(false);
  const [hint, setHint] = useState(undefined);
  const [highPrecisionToggle, setHighPrecisionToggle] = useState(false);
  const [windowPos, setWindowPos] = useState({
    x: 0,
    y: 0,
  });
  const [windowSize, setWindowSize] = useState({
    width: 0,
    height: 0,
  });
  const loadedRef = useRef(false);

  const saveState = useCallback(async () => {
    if (!loadedRef.current) {
      return;
    }
    /*
		let obj = {
			text: text,
			highPrecision: highPrecisionToggle,
			window: {
				position: position,
				size: size,
			},
		};
		const json = JSON.stringify(obj);
		// console.log(`save json = ${json}`);
		await calc.save(json);
		*/
  }, [text, highPrecisionToggle, windowPos, windowSize]);

  useEffect(() => {
    const init = async () => {
      loadState();
      versionUpdate();
      const updateDownloadedHandler = (event, info) => {
        setUpdateAvailable(true);
      };
      window.electronAPI.ipcRenderer.on("has-update", updateDownloadedHandler);

      const onEngineStart = async (event, info) => {
        console.log("Engine started");
        setEngineReady(true);
      };
      window.electronAPI.ipcRenderer.on("on-engine-start", onEngineStart);

      return () => {
        window.electronAPI.ipcRenderer.removeListener(
          "has-update",
          updateDownloadedHandler,
        );
      };
    };
    init();
  }, []);

  useEffect(() => {
    async function check_env() {}
    check_env();
  }, []);

  useEffect(() => {
    calc.precision = highPrecisionToggle ? 7 : 2;
  }, [highPrecisionToggle]);

  const loadState = useCallback(async () => {
    if (loadedRef.current) {
      return;
    }
    const text = localStorage.getItem("text") || "";
    setText(text);
    loadedRef.current = true;
  }, []);

  useEffect(() => {
    const recalcAndSave = async () => {
      if (!engineReady) {
        setValues([]);
        return;
      }
      setValues(await calc.calculate(text));
      await saveState();
    };
    recalcAndSave();
  }, [text, highPrecisionToggle, saveState, engineReady]);

  const versionUpdate = useCallback(async () => {
    const version = await window.electronAPI.getAppVersion();
    setVersion(version);
  }, []);

  const onChangeHandler = useCallback((text) => {
    localStorage.setItem("text", text);
    setText(text);
  }, []);

  const onMinimize = useCallback(() => window.electronAPI.windowControl("minimize"), []);

  const onClose = useCallback(() => window.electronAPI.windowControl("close"), []);

  const onUpdateTap = useCallback(async () => {
    await window.electronAPI.installUpdate();
  }, []);

  const onHint = useCallback((text) => {
    setHint(text);
  }, []);

  const onPrecisionToggle = useCallback(
    (toggleOn) => {
      setHighPrecisionToggle(toggleOn);
    },
    [highPrecisionToggle],
  );

  const onPrecisionToggleMouseEnter = useCallback(() => {
    setHint("High Precision");
  }, []);

  const onPrecisionToggleMouseLeave = useCallback(() => {
    setHint(undefined);
  }, []);

  return (
    <div className="app">
      <div className="titlebar" data-tauri-drag-region>
        <div className="leftControls" />
        <div className="title" data-tauri-drag-region>
          &nbsp;
        </div>
        <div className="rightControls">
          <div id="minimize" className="control" onClick={onMinimize} />
          <div id="close" className="control" onClick={onClose} />
        </div>
      </div>
      <div className="editor_container">
        <MultiEditor
          key="editor"
          text={text}
          values={values}
          onChange={onChangeHandler}
          onHint={onHint}
        />
      </div>
      <div className="bottombar">
        <div className="version">v{version}</div>
        {updateAvailable && !updateDisabled && (
          <div
            className={`update ${hint !== undefined ? "hidden" : ""}`}
            onClick={onUpdateTap}
          >
            Update available
          </div>
        )}
        <div className={`hint ${hint !== undefined ? "" : "hidden"}`}>
          {hint}
        </div>
        <div className="precision">
          <Toggle
            initPosition={highPrecisionToggle}
            onChange={onPrecisionToggle}
            onMouseEnter={onPrecisionToggleMouseEnter}
            onMouseLeave={onPrecisionToggleMouseLeave}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
