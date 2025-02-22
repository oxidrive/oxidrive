/** @type {import('stylelint').Config} */
export default {
	extends: ["stylelint-config-standard-scss", "stylelint-config-html/svelte"],

	rules: {
		"selector-pseudo-class-no-unknown": [
			true,
			{
				ignorePseudoClasses: ["global"],
			},
		],
	},
};
