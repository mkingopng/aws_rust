import http from 'k6/http';
import { sleep, check } from 'k6';

export let options = {
    stages: [
        { duration: '10m', target: 100 },  // ramp-up to 100 users over 10 minutes
        { duration: '20m', target: 100 },  // stay at 100 users for 20 minutes
        { duration: '10m', target: 50 },   // ramp-down to 50 users over 10 minutes
        { duration: '20m', target: 50 },   // stay at 50 users for 20 minutes
        { duration: '10m', target: 200 },  // ramp-up to 200 users over 10 minutes
        { duration: '20m', target: 200 },  // stay at 200 users for 20 minutes
        { duration: '10m', target: 0 },    // ramp-down to 0 users over 10 minutes
        { duration: '10m', target: 500 },  // ramp-up to 500 users over 10 minutes
        { duration: '10m', target: 500 },  // stay at 500 users for 10 minutes
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
