import { useState, useRef, useEffect, useCallback, memo, useMemo } from "react";
import "./MultiEditor.css";

export function MultiEditor({ text, values, onChange, onHint }) {
	const [localText, setLocalText] = useState(text);
	const [renderedLines, setRenderedLines] = useState([]);
	const [separatorIsVisible, setSeparatorIsVisible] = useState(false);
	const [componentWidth, setComponentWidth] = useState(0);
	const inputRef = useRef(null);
	const highlightRef = useRef(null);
	const componentRef = useRef(null);

	const minResultSizePx = 100;
	const maxResultSizePx = 300;
	const relativeResultSize = 0.35;
	const separatorShownForResultLength = 11;

	// Focust on the input
	useEffect(() => {
		inputRef.current.focus();
	}, []);

	// Resize results area based on component size
	useEffect(() => {
		const handleResize = (entries) => {
			const { width } = entries[0].contentRect;
			setComponentWidth(width);
			const resultWidth = Math.min(
				Math.max(width * relativeResultSize, minResultSizePx),
				maxResultSizePx
			);
			componentRef.current.style.setProperty(
				"--result-width",
				`${resultWidth}px`
			);
		};
		const resizeObserver = new ResizeObserver(handleResize);
		if (componentRef.current) {
			resizeObserver.observe(componentRef.current);
		}
		return () => resizeObserver.disconnect(); // Cleanup on component unmount
	}, []);

	useEffect(() => {
		// Had to sync scroll after each render
		// Manual sync on text change may not sync offset properly
		// due to highlight area not being resized at the moment of sync
		syncScroll();
	});

	useEffect(() => {
		if (text !== localText) {
			setLocalText(text);
		}
	}, [text]);

	const lineHandler = useCallback((event) => {
		if (event == "entered") {
			onHint("Tap to copy");
		}
		if (event == "left") {
			onHint(undefined);
		}
		if (event == "copied") {
			onHint("Copied!");
		}
	}, []);

	const onChangeHandler = useCallback((event) => {
		let value = event.target.value;
		setLocalText(value);
		onChange(value);
	});

	const onScrollHandler = useCallback((event) => {
		syncScroll();
	});

	const rendered_lines = useMemo(() => {
		const text_lines = localText.split("\n");
		let hasLenghtyResults = false;
		const result = text_lines.map((text_line, index) => {
			// Make sure to return the result of map
			let line = text_line || " ";
			let value = values.length > index ? values[index].value : "";
			let html = values.length > index ? values[index].html : line;
			html = html || " ";
			if (value && value.length >= separatorShownForResultLength) {
				hasLenghtyResults = true;
			}
			return (
				<CalcLine key={index} text={html} value={value} handler={lineHandler} />
			);
		});
		// Keep the divider out of roomy windows. When the answer column gets
		// close to the editor, the subtle line makes the two columns readable.
		const hasResults = values.some((item) => item.value);
		setSeparatorIsVisible(hasLenghtyResults || (componentWidth > 0 && componentWidth < 430 && hasResults));
		return result;
	}, [localText, values, componentWidth]);

	return (
		<div className="multieditor" ref={componentRef}>
			{separatorIsVisible && <div className="separator" />}
			<pre ref={highlightRef} aria-hidden="true" className="highlights">
				{rendered_lines}
			</pre>
			<textarea
				ref={inputRef}
				className="editor"
				spellCheck="false"
				onChange={onChangeHandler}
				value={localText}
				onScroll={onScrollHandler}
			/>
		</div>
	);

	function syncScroll() {
		highlightRef.current.scrollTop = inputRef.current.scrollTop;
	}

	function CalcLine({ text, value, handler }) {
		const [isMouseDown, setMouseDown] = useState(false);

		const onMouseDown = useCallback(() => {
			setMouseDown(true);
		});

		const onMouseUp = useCallback(() => {
			try {
				navigator.clipboard.writeText(value);
			} catch (error) {
				console.error("Failed to copy: ", error);
			}
			setMouseDown(false);
			handler("copied");
		});

		const onMouseEnter = useCallback(() => {
			handler("entered");
		});

		const onMouseLeave = useCallback(() => {
			setMouseDown(false);
			handler("left");
		});

		const valueClassNames = `value ${isMouseDown ? "pressed" : ""}`;

		return (
			<span className="line">
				<span
					className="input"
					dangerouslySetInnerHTML={{ __html: text }}
				></span>
				<span className="result">
					<span
						className={valueClassNames}
						onMouseDown={onMouseDown}
						onMouseUp={onMouseUp}
						onMouseEnter={onMouseEnter}
						onMouseLeave={onMouseLeave}
					>
						{value}
					</span>
				</span>
			</span>
		);
	}
}
