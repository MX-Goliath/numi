export class AppCalc {
	debounceTimer = undefined;
	calculateText = undefined;
	precision = 2;

	constructor(invoker) {
		this.invoker = invoker;
	}

	calculate_debounced(text, callback) {
		this.calculateText = text;
		clearTimeout(this.debounceTimer);
		this.debounceTimer = setTimeout(async () => {
			let results = await this.calculate(text);
			if (this.calculateText === text) {
				callback(results);
			}
		}, this.debounceRecalcMs);
	}

	async calculate(text) {
		const lines = text.split("\n");
		const input = {
			input: lines,
			options: {
				precision: this.precision,
			},
		};
		let invoke_result = {};
		invoke_result = await this.invoker("calc_json", {
			text: JSON.stringify(input),
		});
		const result_obj = invoke_result ? JSON.parse(invoke_result) : { result: [] };
		let result = [];
		for (let i = 0; i < lines.length; i++) {
			let value = "";
			let html = "";
			if (i < result_obj.result.length) {
				value = result_obj.result[i].value;
				html = formatArrayToHTMLString(result_obj.result[i].text);
			}
			const wave = calculateWaveLine(lines[i], this.precision);
			if (wave) {
				value = wave;
				html = lines[i];
			}
			result.push({ value: value != "?" ? value : "", html: html });
		}
		return result;
	}

	async save(json) {
		await this.invoker("save_state", { json: json || "" });
	}

	async load() {
		return await this.invoker("get_state");
	}
}

// Keep wave conversions available in a Windows package built on Linux when a
// Windows Rust toolchain is not present. The native engine handles these too;
// this exact-expression path is a small compatibility layer for the unpacked
// Windows build and uses the same vacuum-wave formulas as the Rust converter.
function calculateWaveLine(line, precision) {
		const match = line.match(/^\s*([+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?)\s+(.+?)\s+in\s+(.+?)\s*$/i);
	if (!match) return undefined;
	const number = Number(match[1]);
	if (!Number.isFinite(number)) return undefined;
	const source = waveUnit(match[2]);
	const target = waveUnit(match[3]);
	if (!source || !target) return undefined;
	if (source.kind === "length" && target.kind === "length") return undefined;
	if (source.kind === "time" && target.kind === "time") return undefined;
	if (source.kind === "wavelength" && target.kind === "wavelength") return undefined;
	const c = 299792458;
	const tau = 2 * Math.PI;
	let frequency;
	if (source.kind === "frequency") frequency = number * source.factor;
	else if (source.kind === "period") frequency = 1 / number;
	else if (source.kind === "time") frequency = 1 / (number * source.factor);
	else if (source.kind === "wavelength") frequency = c / number;
	else if (source.kind === "length") frequency = c / (number * source.factor);
	else if (source.kind === "angular") frequency = number / tau;
	else if (source.kind === "wave_number") frequency = number * c / tau;
	if (!Number.isFinite(frequency)) return undefined;

	let result;
	let unit;
	if (target.kind === "frequency") {
		result = frequency / target.factor;
		unit = target.unit;
	} else if (target.kind === "period") {
		result = 1 / frequency;
		unit = "s";
	} else if (target.kind === "time") {
		result = 1 / frequency / target.factor;
		unit = target.unit;
	} else if (target.kind === "wavelength") {
		result = c / frequency;
		unit = "m";
	} else if (target.kind === "length") {
		result = c / frequency / target.factor;
		unit = target.unit;
	} else if (target.kind === "angular") {
		result = tau * frequency;
		unit = "rad/s";
	} else if (target.kind === "wave_number") {
		result = tau * frequency / c;
		unit = "rad/m";
	} else {
		return undefined;
	}
	return `${formatWaveNumber(result, precision)} ${unit}`;
}

function waveUnit(value) {
	const raw = value.trim().replace(/\s+/g, " ");
	const unit = raw.toLowerCase();
	if (["hz", "hertz", "frequency", "frequencies", "cps", "cycles per second"].includes(unit)) return { kind: "frequency", factor: 1, unit: "Hz" };
	const frequencyPrefixes = { mHz: 1e-3, "μHz": 1e-6, nHz: 1e-9, kHz: 1e3, MHz: 1e6, GHz: 1e9, THz: 1e12 };
	if (frequencyPrefixes[raw]) return { kind: "frequency", factor: frequencyPrefixes[raw], unit: raw };
	const lowercaseFrequencyPrefixes = {
		khz: [1e3, "kHz"], mhz: [1e6, "MHz"], ghz: [1e9, "GHz"], thz: [1e12, "THz"], nhz: [1e-9, "nHz"],
	};
	if (lowercaseFrequencyPrefixes[unit]) return { kind: "frequency", factor: lowercaseFrequencyPrefixes[unit][0], unit: lowercaseFrequencyPrefixes[unit][1] };
	if (["period", "periods", "wave period", "wave periods", "t", "seconds per cycle"].includes(unit)) return { kind: "period" };
	if (["wavelength", "wavelengths", "lambda", "λ"].includes(unit)) return { kind: "wavelength" };
	if (["angular frequency", "angular frequencies", "omega", "ω"].includes(unit)) return { kind: "angular" };
	if (["wave number", "wave numbers", "wavenumber", "wavenumbers", "k"].includes(unit)) return { kind: "wave_number" };
	const lengths = { m: ["m", 1], mm: ["mm", 1e-3], cm: ["cm", 1e-2], km: ["km", 1e3], inch: ["inch", 0.0254], ft: ["ft", 0.3048], mile: ["mile", 1609.344] };
	if (lengths[unit]) return { kind: "length", unit: lengths[unit][0], factor: lengths[unit][1] };
	const times = {
		s: ["s", 1], sec: ["sec", 1], second: ["sec", 1], seconds: ["sec", 1],
		ms: ["ms", 1e-3], msec: ["ms", 1e-3], millisecond: ["ms", 1e-3], milliseconds: ["ms", 1e-3],
		us: ["μs", 1e-6], "μs": ["μs", 1e-6], nsec: ["ns", 1e-9], ns: ["ns", 1e-9],
		min: ["min", 60], minute: ["min", 60], minutes: ["min", 60],
		h: ["h", 3600], hr: ["hr", 3600], hour: ["hr", 3600], hours: ["hr", 3600],
		day: ["d", 86400], days: ["d", 86400], week: ["week", 604800], weeks: ["week", 604800],
		month: ["month", 18144000], months: ["month", 18144000], yr: ["yr", 220752000], year: ["yr", 220752000], years: ["yr", 220752000],
	};
	if (times[unit]) return { kind: "time", unit: times[unit][0], factor: times[unit][1] };
	return undefined;
}

function formatWaveNumber(number, precision) {
	if (Math.abs(number - Math.round(number)) < 1e-10) return String(Math.round(number));
	return number.toFixed(precision);
}

function formatArrayToHTMLString(arr) {
	return arr
		.map((item) => {
			if (item.stype) {
				return `<span class="sh_${item.stype.toLowerCase()}">${
					item.string
				}</span>`;
			} else {
				return item.string;
			}
		})
		.join("");
}
