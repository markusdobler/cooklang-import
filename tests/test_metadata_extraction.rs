use cooklang_import::converters::{parse_converter_output, ConversionResult, ConversionMetadata, TokenUsage};

#[test]
fn test_parse_converter_output_with_metadata() {
    let output = r#"---
title: Chocolate Chip Cookies
prep_time: 15 minutes
cook_time: 12 minutes
servings: 24 cookies
---

Mix @flour{2%cups} with @sugar{1%cup}.

Bake at 350°F for ~{12%minutes}."#;

    let (metadata, content) = parse_converter_output(output);

    assert_eq!(metadata.get("title"), Some(&"Chocolate Chip Cookies".to_string()));
    assert_eq!(metadata.get("prep_time"), Some(&"15 minutes".to_string()));
    assert_eq!(metadata.get("cook_time"), Some(&"12 minutes".to_string()));
    assert_eq!(metadata.get("servings"), Some(&"24 cookies".to_string()));
    assert!(content.contains("@flour{2%cups}"));
    assert!(content.contains("~{12%minutes}"));
}

#[test]
fn test_parse_converter_output_with_empty_metadata() {
    let output = r#"---
title: 
prep_time: 
cook_time: 
servings: 
---

Mix @flour{2%cups} with @sugar{1%cup}."#;

    let (metadata, content) = parse_converter_output(output);

    // Empty values should not be added to metadata
    assert!(metadata.is_empty());
    assert!(content.contains("@flour{2%cups}"));
}

#[test]
fn test_parse_converter_output_without_frontmatter() {
    let output = "Mix @flour{2%cups} with @sugar{1%cup}.";

    let (metadata, content) = parse_converter_output(output);

    assert!(metadata.is_empty());
    assert_eq!(content, output);
}

#[test]
fn test_parse_converter_output_partial_metadata() {
    let output = r#"---
title: Quick Pasta
cook_time: 20 minutes
---

Cook @pasta{500%g} in boiling water."#;

    let (metadata, content) = parse_converter_output(output);

    assert_eq!(metadata.get("title"), Some(&"Quick Pasta".to_string()));
    assert_eq!(metadata.get("cook_time"), Some(&"20 minutes".to_string()));
    assert_eq!(metadata.get("prep_time"), None);
    assert_eq!(metadata.get("servings"), None);
    assert!(content.contains("@pasta{500%g}"));
}

#[test]
fn test_conversion_result_with_extracted_metadata() {
    let mut metadata_map = std::collections::HashMap::new();
    metadata_map.insert("title".to_string(), "Test Recipe".to_string());
    metadata_map.insert("servings".to_string(), "4".to_string());

    let result = ConversionResult {
        content: "Cook @pasta{500%g}".to_string(),
        metadata: ConversionMetadata {
            model_version: Some("gpt-4o-mini".to_string()),
            tokens_used: TokenUsage {
                input_tokens: Some(100),
                output_tokens: Some(50),
            },
            latency_ms: 1000,
        },
        extracted_metadata: Some(metadata_map),
    };

    assert!(result.extracted_metadata.is_some());
    let extracted = result.extracted_metadata.unwrap();
    assert_eq!(extracted.get("title"), Some(&"Test Recipe".to_string()));
    assert_eq!(extracted.get("servings"), Some(&"4".to_string()));
}
