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

context += render(Archetype("model-casing"), ANSWERS + #{
    default_model: context["project-prefix"],
});

if switch_enabled("debug-context") {
    display(as_yaml(context));
}

render(Directory("contents/base"), context);
for entity in context.model.entities.values() {
    render(Directory("contents/entity_persistence"), context + entity);
}

let protobuf_archetype = Archetype("generate-protobuf");
render(protobuf_archetype, Path(`${context["project-name"]}/specs/self`), context);

if context.applications != () {
    for application in context.applications.values() {
        render(protobuf_archetype, Path(`${context["project-name"]}/specs/clients/`), application);
    }
}

// return context
context