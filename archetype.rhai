let context = #{};

context += prompt("Project Prefix:", "project-prefix", #{
    default_value: "",
    cases: CasedIdentityCasedValue(PROGRAMMING_CASES),
});

context += prompt("Project Suffix:", "project-suffix", #{
    defaults_with: "Service",
    cases: CasedIdentityCasedValue(PROGRAMMING_CASES),
});

context += set("project-name", render("{{ project-prefix }}-{{project-suffix}}", context), #{
    cases: [
        CasedIdentityCasedValue(PROGRAMMING_CASES),
        FixedKeyCasedValue("project-title", KebabCase),
    ],
});

context += prompt("Service Port:", "service-port", #{
    defaults_with: 8080,
    type: Int,
    min: 1024,
    max: 65534,
});

render(Directory("contents/base"), context);

// return context
context