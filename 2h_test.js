import http from 'k6/http';
import { sleep, check } from 'k6';

export let options = {
    stages: [
        { duration: '5m', target: 10 },  // ramp-up to 10 users over 5 minutes
        { duration: '10m', target: 10 },  // stay at 10 users for 10 minutes
        { duration: '5m', target: 5 },   // ramp-down to 5 users over 5 minutes
        { duration: '10m', target: 5 },   // stay at 5 users for 10 minutes
        { duration: '5m', target: 10 },  // ramp-up to 10 users over 5 minutes
        { duration: '10m', target: 10 },  // stay at 10 users for 10 minutes
        { duration: '5m', target: 0 },    // ramp-down to 0 users over 5 minutes
        { duration: '5m', target: 10 },  // ramp-up to 10 users over 5 minutes
        { duration: '10m', target: 10 },  // stay at 10 users for 10 minutes
    ],
};

const url = 'https://3si1yzcmgj.execute-api.ap-southeast-2.amazonaws.com/rusty_dev/rust-resource';

export default function () {
    let res = http.get(url);
    check(res, {
        'status is 200': (r) => r.status === 200,
    });
    sleep(1);
}
