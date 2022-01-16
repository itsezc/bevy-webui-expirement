module.exports = {
	content: {
		files: [
			'./src/**/*.rs',
			'./index.html'
		],
		extract: {
			rs: (content) => {
				const customRegex = /(?<=classes!\((?:"[^"]*",\s*)*")[^"]*(?=")/g;

				const result = content.match(customRegex) || [];

				let results = [
					...result
				].flat().filter((v) =>v !== undefined);

				return results;
			}
		}
	},
	theme: {},
	variants: {},
	plugins: []
};