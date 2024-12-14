const patoka = require('patoka');
const Connection = patoka.Connection;
const TaskManager = patoka.TaskManager;

const taskName = 'master';
const logger = patoka.Logger.getLogger(taskName);

function extractCountries() {
    const countryElems = document.querySelectorAll('a');

    let countries = [];
    for (const ce of countryElems) {
        if (!ce.hasAttribute('href') || !ce.hasAttribute('title'))
            continue;

        countries.push({
            name: ce.getAttribute('title'),
            href: window.location + ce.getAttribute('href'),
        });
    }

    return countries;
}

execute = async (ctx) => {
    try {
        const params = ctx.params;
        const page = ctx.page;
        const taskUuid = ctx.taskUuid;

        if (!params) {
            throw "No params provided";
        }

        if (!params.url)
            throw "No params.url provided";

        await page.goto(params.url);

        const countries = await page.evaluate(extractCountries);

        logger.debug("Extracted countries: " + JSON.stringify(countries));

        Connection.sendTaskResult(
            taskUuid,
            { 'countries': countries }
        );

        TaskManager.completed(taskUuid);
    }
    catch (e) {
        Connection.sendMsgError("Failed to execute task: " + e);
    }
}

module.exports = {
    execute
}
