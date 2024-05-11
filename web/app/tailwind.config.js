/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "all",
	content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
	theme: {
		extend: {
			colors: {
				primary: {
					50: "#e8edf2",
					100: "#d1dbe4",
					200: "#a3b7ca",
					300: "#7592af",
					400: "#476e95",
					500: "#194a7a",
					600: "#143b62",
					700: "#0f2c49",
					800: "#0a1e31",
					900: "#050f18",
				},
				danger: {
					50: "#fbedeb",
					100: "#f7dbd6",
					200: "#efb6ad",
					300: "#e69285",
					400: "#de6d5c",
					500: "#d64933",
					600: "#ab3a29",
					700: "#802c1f",
					800: "#561d14",
					900: "#2b0f0a",
				},
				success: {
					50: "#f0feee",
					100: "#e1fedd",
					200: "#c3fdbc",
					300: "#a6fb9a",
					400: "#88fa79",
					500: "#6af957",
					600: "#55c746",
					700: "#409534",
					800: "#2a6423",
					900: "#153211",
				},
				flexGrow: {
					2: "2",
				},
			},
		},
		fontFamily: {
			sans: ["Roboto", "sans-serif"],
		},
		fontWeight: {
			light: "300",
			regular: "400",
			bold: "500",
		},
	},
	plugins: [],
	safelist: [
		// Title
		"text-sm",
		"text-md",
		"text-lg",
		"text-xl",
	],
};
