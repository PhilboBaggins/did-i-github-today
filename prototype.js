'use strict';

var request = require('request');
var _ = require('underscore'); // http://underscorejs.org/

function getDates(obj) {
    return new Date(obj.created_at);
}

function isToday(date) {
    const today = new Date();
    return date.getYear() === today.getYear() &&
        date.getMonth() === today.getMonth() &&
        date.getDay() === today.getDay();
}

const USER_NAME = 'philbobaggins'; // TODO: Get from command line arguments

const options = {
    uri: 'https://api.github.com/users/' + USER_NAME + '/events',
    headers: { 'User-Agent': 'did-i-github-today-prototype' }
};

request(options, function(error, response, body) {
    // TODO: Check response.statusCode and error variables

    const events = JSON.parse(body);
    const dates = _.map(events, getDates);
    const todaysEvents = _.filter(dates, isToday);

    if (_.any(todaysEvents)) {
        console.log('Yes');
    } else {
        console.log('No');
    }
});