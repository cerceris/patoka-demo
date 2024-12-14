const patoka = require('patoka');
const Connection = patoka.Connection;
const TaskManager = patoka.TaskManager;

const taskName = 'subtask';
const logger = patoka.Logger.getLogger(taskName);

function extractPopulation() {
    const rowElems = document.querySelectorAll('tr > td');

    if (rowElems.length < 2)
        return 0;

    const it = rowElems[1].innerText;
    let population = parseInt(it);

    return population;
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

        if (!params.name)
            throw "No params.name provided";

        logger.debug("Open country page " + params.name + ": " + params.url);

        await page.goto(params.url);

        let population = await page.evaluate(extractPopulation);

        if (isNaN(population))
            population = 0;

        logger.debug(params.name + " population: " + population);

        Connection.sendTaskResult(
            taskUuid,
            { 'population': population }
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
