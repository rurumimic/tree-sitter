module.exports = grammar({
  name: "impala",

  rules: {
    source_file: $ => repeat($._statement),

    _statement: $ => seq(choice($.select_statement), optional(";")),

    select_statement: $ =>
      seq($.select, "*", optional($.from), optional($.table_name)),

    select: $ => new RegExp("select|SELECT"),
    from: $ => new RegExp("from|FROM"),
    table_name: $ => "table_name",
  },
});
