#!/usr/bin/env node

'use strict';

const request = require('request'); // https://github.com/request/request
const _ = require('underscore'); // http://underscorejs.org/

const verbose = ((process.argv.indexOf('-v') > 0) || (process.argv.indexOf('--verbose') > 0));

function isToday(obj) {
    const today = new Date();
    return (new Date(obj.created_at)).getYear() === today.getYear() &&
        (new Date(obj.created_at)).getMonth() === today.getMonth() &&
        (new Date(obj.created_at)).getDate() === today.getDate();
}

const USER_NAME = 'philbobaggins'; // TODO: Get from command line arguments

const options = {
    uri: 'https://api.github.com/users/' + USER_NAME + '/events',
    headers: { 'User-Agent': 'did-i-github-today-prototype' }
};

request(options, function(error, response, body) {
    // TODO: Check response.statusCode and error variables

    const events = JSON.parse(body);
    const todaysEvents = _.filter(events, isToday);

    if (verbose) {
        for (var i = 0; i < todaysEvents.length; i++) {
            console.log(todaysEvents[i].type +' at ' +  todaysEvents[i].created_at);
        }
    }

    if (_.any(todaysEvents)) {
        console.log('Yes');
    } else {
        console.log('No');
    }
});
