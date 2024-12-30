const express = require('express');
const path = require('path');
const snippets = require('./snippets.json');

const app = express();
app.set('view engine', 'ejs');
app.set('views', path.join(__dirname, 'views'));
app.use(express.static(path.join(__dirname, 'public')));

app.get('/', (req, res) => {
    res.redirect('/snippet/1');
});

app.get('/snippet/:id', (req, res) => {
    const snippetId = parseInt(req.params.id);
    const snippet = snippets.find(s => s.id === snippetId);
    if (!snippet) return res.status(404).send('Snippet not found');

    // Find the index of the current snippet
    const currentIndex = snippets.findIndex(s => s.id === snippetId);

    // Calculate the previous and next snippet IDs based on the current index
    const previousSnippetId = currentIndex > 0 ? snippets[currentIndex - 1].id : snippetId;  // Stay on the first for Previous
    const nextSnippetId = currentIndex < snippets.length - 1 ? snippets[currentIndex + 1].id : snippetId;  // Stay on the last for Next

    // Add line numbers to the snippet code
    const snippetWithLineNumbers = addLineNumbers(snippet.snippet);
    const findingDescription = getFindingDescription(snippet.vulnerability);

    res.render('index', {
        snippetWithLineNumbers,
        file: snippet.file,
        vulnerability: snippet.vulnerability,
        vulnerableLine: snippet.vulnerableLine,
        findingDescription,
        previousSnippetId,
        nextSnippetId
    });
});

function addLineNumbers(code) {
    const lines = code.split('\n');
    return lines.map((line, index) => {
        return `${index + 1}: ${line}`;
    }).join('\n');
}

function getFindingDescription(vulnerability) {
    switch (vulnerability) {
        case 'SQL Injection':
            return 'The code is vulnerable to SQL Injection because user input is directly concatenated into the SQL query.';
        case 'Hardcoded Credentials':
            return 'The code contains hardcoded credentials, which pose a security risk if exposed.';
        default:
            return 'Description not available.';
    }
}

app.listen(3000, () => {
    console.log('App running at http://localhost:3000');
});
