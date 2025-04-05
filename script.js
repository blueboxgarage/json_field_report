document.addEventListener('DOMContentLoaded', function() {
    const json1Element = document.getElementById('json1');
    const json2Element = document.getElementById('json2');
    const generateReportBtn = document.getElementById('generate-report');
    const reportTableBody = document.querySelector('#report-table tbody');

    generateReportBtn.addEventListener('click', function() {
        try {
            // Parse the JSON inputs
            const json1 = JSON.parse(json1Element.value);
            const json2 = JSON.parse(json2Element.value);
            
            // Generate the report
            const differences = compareJsonStructures(json1, json2);
            
            // Update the table
            updateReportTable(differences);
        } catch (error) {
            alert('Error: ' + error.message);
        }
    });

    function compareJsonStructures(json1, json2) {
        const paths1 = extractPaths(json1);
        const paths2 = extractPaths(json2);
        
        const differences = [];
        
        // Fields in json1 but missing in json2
        for (const path of paths1) {
            if (!paths2.includes(path)) {
                differences.push({
                    field: path,
                    missingFrom: 'json2'
                });
            }
        }
        
        // Fields in json2 but missing in json1
        for (const path of paths2) {
            if (!paths1.includes(path)) {
                differences.push({
                    field: path,
                    missingFrom: 'json1'
                });
            }
        }
        
        return differences;
    }

    function extractPaths(obj, prefix = '') {
        const paths = [];
        
        if (typeof obj !== 'object' || obj === null) {
            return paths;
        }
        
        if (Array.isArray(obj)) {
            // For arrays, we just check the first element's structure
            if (obj.length > 0) {
                const arrayPath = prefix ? `${prefix}[]` : '[]';
                paths.push(arrayPath);
                paths.push(...extractPaths(obj[0], arrayPath));
            } else {
                // Empty array
                if (prefix) {
                    paths.push(prefix);
                }
            }
        } else {
            // Object
            if (Object.keys(obj).length === 0 && prefix) {
                paths.push(prefix);
            }
            
            for (const [key, value] of Object.entries(obj)) {
                const newPrefix = prefix ? `${prefix}.${key}` : key;
                paths.push(newPrefix);
                paths.push(...extractPaths(value, newPrefix));
            }
        }
        
        return paths;
    }

    function updateReportTable(differences) {
        // Clear the existing rows
        reportTableBody.innerHTML = '';
        
        // Add the new rows
        for (const diff of differences) {
            const row = document.createElement('tr');
            
            const fieldCell = document.createElement('td');
            fieldCell.textContent = diff.field;
            row.appendChild(fieldCell);
            
            const missingFromCell = document.createElement('td');
            missingFromCell.textContent = diff.missingFrom;
            row.appendChild(missingFromCell);
            
            reportTableBody.appendChild(row);
        }
    }
});