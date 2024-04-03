function shal_language() {
// Create your own language definition here
// You can safely look at other samples without losing modifications.
// Modifications are not saved on browser refresh/close though -- copy often!
    return {
        // Set defaultToken to invalid to see what you do not tokenize yet
        defaultToken: 'invalid',

        keywords: [
            'if', 'else', 'on',
            'redge', 'fedge',
            'toggle', 'set',
            'high', 'low'
        ],

        typeKeywords: [
            'input', 'output', 'entity'
        ],

        operators: [
            'or', 'and', 'is', 'was', 'xor', 'not'
        ],

        brackets: [
            ['{', '}', 'delimiter.curly']
        ],

        // The main tokenizer for our languages
        tokenizer: {
            root: [
                // identifiers and keywords
                [/[a-zA-Z][a-zA-Z0-9_]*/, {
                    cases: {
                        '@typeKeywords': 'type',
                        '@keywords': 'keyword',
                        '@operators': 'operator',
                        '@default': 'identifier'
                    }
                }],

                // whitespace
                {include: '@whitespace'},

                // delimiters
                [/[{}]/, '@brackets'],

                // assignment
                [/=/, 'operator'],

                // numbers
                [/[0-9]+/, 'number'],

                // delimiter
                [/[;]/, 'delimiter'],
            ],

            whitespace: [
                [/[ \t\r\n]+/, 'white'],
                [/\/\/.*$/, 'comment'],
            ],
        },
    };
}
