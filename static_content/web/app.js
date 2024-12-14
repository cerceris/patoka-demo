const express = require('express');

const app = express();

app.set('views', './views');
app.set('view engine', 'pug');

// Top 10 2018 Population from https://www.internetworldstats.com/stats8.htm.
const countries = {
    'china': { name: 'China', population: 1415045928 },
    'india': { name: 'India', population: 1354051854 },
    'us': { name: 'United States', population: 326766748 },
    'indonesia': { name: 'Indonesia', population: 266794980 },
    'brazil': { name: 'Brazil', population: 210867954 },
    'pakistan': { name: 'Pakistan', population: 200813818 },
    'nigeria': { name: 'Nigeria', population: 195875237 },
    'bangladesh': { name: 'Bangladesh', population: 166368149 },
    'russia': { name: 'Russia', population: 143964709 },
    'mexico': { name: 'Mexico', population: 130759074 },
};

app.get('/', (req, res) => {
    res.render('index', { title: 'Static Content', countries: countries });
});

app.get('/:country', (req, res) => {
    const country = countries[req.params.country];
    if (country === undefined) {
        res.send("Not found.");
    } else {
        res.render('country', { country: country });
    }
});

const port = 5000;

app.listen(port, ()=> {
    console.log(`Listening on port ${port}`);
});
