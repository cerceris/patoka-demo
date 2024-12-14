const patoka = require('patoka');
const Connection = patoka.Connection;
const TaskManager = patoka.TaskManager;

const logger = patoka.Logger.logger;

execute = async (ctx) => {

    try {
        const params = ctx.params;
        const taskUuid = ctx.taskUuid;

        if (!params) {
            throw "No params provided";
        }

        if (!params.number)
            throw "No params.number provided";

        const number = params.number;

        logger.debug(`number: ${number}`);

        const result = number * number;

        logger.debug('Result: ' + result);

        Connection.sendTaskResult(
            taskUuid,
            { 'square': result }
        );

        ctx.stop()
    }
    catch (e) {
        Connection.sendMsgError("Failed to execute task: " + e);
    }
}

module.exports = {
    execute
}
