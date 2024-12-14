const patoka = require('patoka');
const Connection = patoka.Connection;
const TaskManager = patoka.TaskManager;

const logger = patoka.Logger.logger;

const Chance = require('chance');
let chance = new Chance();

execute = async (ctx) => {

    try {
        const params = ctx.params;
        const taskUuid = ctx.taskUuid;

        if (!params) {
            throw "No params provided";
        }

        if (!params.max)
            throw "No params.max provided";

        if (!params.number)
            throw "No params.number provided";

        const max = params.max;
        const number = params.number;

        logger.debug(`max: ${max}; number: ${number}`);

        let generatedNumbers = [];
        for (let i = 0; i < number; i++) {
            generatedNumbers.push(chance.integer({ min: 1, max }));
        }

        logger.debug('Result: ' + generatedNumbers);

        Connection.sendTaskResult(
            taskUuid,
            { 'generated_numbers': generatedNumbers }
        );

        ctx.stop();
    }
    catch (e) {
        Connection.sendMsgError("Failed to execute task: " + e);
    }
}

module.exports = {
    execute
}
