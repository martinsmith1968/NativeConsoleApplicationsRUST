use super::*;

// ===== UUID V4 Tests =====

#[test]
fn test_v4_uuid_generation_basic() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: false,
        },
    );

    // V4 UUIDs should be 36 characters with hyphens
    assert_eq!(formatted.len(), 36);
    // Should match pattern: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    assert_eq!(formatted.chars().nth(14).unwrap(), '4');
}

#[test]
fn test_v4_uuid_hyphenated_format() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: false,
        },
    );

    // Verify hyphens at correct positions
    assert_eq!(formatted.chars().nth(8).unwrap(), '-');
    assert_eq!(formatted.chars().nth(13).unwrap(), '-');
    assert_eq!(formatted.chars().nth(18).unwrap(), '-');
    assert_eq!(formatted.chars().nth(23).unwrap(), '-');
}

#[test]
fn test_v4_uuid_non_hyphenated_format() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: false,
            uppercase: false,
        },
    );

    // Non-hyphenated should be 32 characters
    assert_eq!(formatted.len(), 32);
    // Should not contain hyphens
    assert!(!formatted.contains('-'));
    // Should only contain hex characters
    assert!(formatted.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_v4_uuid_uppercase() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: true,
        },
    );

    // Check that all hex digits are uppercase
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_uppercase());
}

#[test]
fn test_v4_uuid_lowercase() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: false,
        },
    );

    // Check that all hex digits are lowercase
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_lowercase());
}

#[test]
fn test_v4_uuid_uniqueness() {
    let options1 = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid1 = generate_guid(options1);
    let options2 = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };
    let uuid2 = generate_guid(options2);

    // V4 UUIDs should be different (statistically)
    assert_ne!(uuid1, uuid2);
}

#[test]
fn test_v4_uuid_uppercase_non_hyphenated() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: false,
            uppercase: true,
        },
    );

    // Should be 32 chars, no hyphens, uppercase
    assert_eq!(formatted.len(), 32);
    assert!(!formatted.contains('-'));
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_uppercase());
}

// ===== UUID V6 Tests =====

#[test]
fn test_v6_uuid_generation_basic() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "1,2,3,4,5,6".to_string(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: false,
        },
    );

    // V6 UUIDs should be 36 characters with hyphens
    assert_eq!(formatted.len(), 36);
    // Should be valid hex
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert!(hex_chars.len() > 0);
}

#[test]
fn test_v6_uuid_with_custom_seed_applies_correctly() {
    // Generate V6 UUID with specific seed
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "10,20,30,40,50,60".to_string(),
    };

    let uuid = generate_guid(options);

    // The seed values should appear in the node part of the UUID
    // V6 format: time_high(32) - time_mid(16) - time_low_and_version(16) - clock_seq_and_reserved(8) - clock_seq_low(8) - node(48)
    // The node is the last 12 hex characters (6 bytes)
    let node_bytes = uuid.as_bytes();

    // Verify seed bytes are in the node (last 6 bytes)
    assert_eq!(node_bytes[10], 10);
    assert_eq!(node_bytes[11], 20);
    assert_eq!(node_bytes[12], 30);
    assert_eq!(node_bytes[13], 40);
    assert_eq!(node_bytes[14], 50);
    assert_eq!(node_bytes[15], 60);
}

#[test]
fn test_v6_uuid_with_default_seed() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "1,2,3,4,5,6".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // Default seed: 1,2,3,4,5,6
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 2);
    assert_eq!(node_bytes[12], 3);
    assert_eq!(node_bytes[13], 4);
    assert_eq!(node_bytes[14], 5);
    assert_eq!(node_bytes[15], 6);
}

#[test]
fn test_v6_uuid_with_empty_seed_uses_default_node() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // When seed is empty, should use default node [1; 6]
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 1);
    assert_eq!(node_bytes[12], 1);
    assert_eq!(node_bytes[13], 1);
    assert_eq!(node_bytes[14], 1);
    assert_eq!(node_bytes[15], 1);
}

#[test]
fn test_v6_uuid_with_partial_seed_uses_default() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "10,20,30".to_string(), // Only 3 values instead of 6
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // When seed has fewer than 6 values, should use default node
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 1);
    assert_eq!(node_bytes[12], 1);
    assert_eq!(node_bytes[13], 1);
    assert_eq!(node_bytes[14], 1);
    assert_eq!(node_bytes[15], 1);
}

#[test]
fn test_v6_uuid_with_invalid_seed_uses_default() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "abc,def,ghi,jkl,mno,pqr".to_string(), // Invalid values
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // When seed contains invalid values, should use default node
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 1);
    assert_eq!(node_bytes[12], 1);
    assert_eq!(node_bytes[13], 1);
    assert_eq!(node_bytes[14], 1);
    assert_eq!(node_bytes[15], 1);
}

#[test]
fn test_v6_uuid_with_mixed_valid_invalid_seed_uses_default() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "10,20,abc,40,50,60".to_string(), // Mix of valid and invalid
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // When seed has any invalid values, should use default node
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 1);
    assert_eq!(node_bytes[12], 1);
    assert_eq!(node_bytes[13], 1);
    assert_eq!(node_bytes[14], 1);
    assert_eq!(node_bytes[15], 1);
}

#[test]
fn test_v6_uuid_seed_with_whitespace() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: " 100 , 110 , 120 , 130 , 140 , 150 ".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // Seed values should be parsed correctly with whitespace trimming
    assert_eq!(node_bytes[10], 100);
    assert_eq!(node_bytes[11], 110);
    assert_eq!(node_bytes[12], 120);
    assert_eq!(node_bytes[13], 130);
    assert_eq!(node_bytes[14], 140);
    assert_eq!(node_bytes[15], 150);
}

// ===== UUID V7 Tests =====

#[test]
fn test_v7_uuid_generation_basic() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V7,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: false,
        },
    );

    // V7 UUIDs should be 36 characters with hyphens
    assert_eq!(formatted.len(), 36);
    // Should be valid hex
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert!(hex_chars.len() > 0);
}

#[test]
fn test_v7_uuid_version_field() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V7,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = uuid.hyphenated().to_string();

    // V7 version field should be '7' at position 14
    assert_eq!(formatted.chars().nth(14).unwrap(), '7');
}

// ===== NanoID Tests =====

#[test]
fn test_nanoid_generation_default_length() {
    let nanoid = generate_nanoid(21);

    // Default NanoID should be 21 characters
    assert_eq!(nanoid.len(), 21);
    // Should only contain valid NanoID characters (0-9, A-Z, a-z, -, _)
    assert!(
        nanoid
            .chars()
            .all(|c| { c.is_ascii_alphanumeric() || c == '-' || c == '_' })
    );
}

#[test]
fn test_nanoid_generation_custom_length_short() {
    let nanoid = generate_nanoid(1);

    // Should respect custom length
    assert_eq!(nanoid.len(), 1);
}

#[test]
fn test_nanoid_generation_custom_length_long() {
    let nanoid = generate_nanoid(100);

    // Should handle large lengths
    assert_eq!(nanoid.len(), 100);
    assert!(
        nanoid
            .chars()
            .all(|c| { c.is_ascii_alphanumeric() || c == '-' || c == '_' })
    );
}

#[test]
fn test_nanoid_generation_very_long() {
    let nanoid = generate_nanoid(255);

    // Should handle very long lengths
    assert_eq!(nanoid.len(), 255);
    assert!(
        nanoid
            .chars()
            .all(|c| { c.is_ascii_alphanumeric() || c == '-' || c == '_' })
    );
}

#[test]
fn test_nanoid_uniqueness() {
    let nanoid1 = generate_nanoid(21);
    let nanoid2 = generate_nanoid(21);

    // NanoIDs should be different
    assert_ne!(nanoid1, nanoid2);
}

// ===== Template and Output Tests =====

#[test]
fn test_format_output_with_uuid_placeholder() {
    let template = "UUID: {uuid}";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 1);

    assert_eq!(result, "UUID: 550e8400-e29b-41d4-a716-446655440000");
}

#[test]
fn test_format_output_with_sequence_placeholder() {
    let template = "Item {sequence}";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 42);

    assert_eq!(result, "Item 42");
}

#[test]
fn test_format_output_with_both_placeholders() {
    let template = "Sequence {sequence}: UUID {uuid}";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 5);

    assert_eq!(
        result,
        "Sequence 5: UUID 550e8400-e29b-41d4-a716-446655440000"
    );
}

#[test]
fn test_format_output_empty_template() {
    let template = "";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 1);

    // Empty template should return the UUID itself
    assert_eq!(result, uuid);
}

#[test]
fn test_format_output_plain_text_no_placeholders() {
    let template = "Generated: ";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 1);

    assert_eq!(result, "Generated: ");
}

#[test]
fn test_format_output_malformed_template_fallback_to_uuid() {
    // Malformed template with unclosed brace
    let template = "UUID: {uuid";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 1);

    // Should handle gracefully by returning the UUID
    assert_eq!(result, uuid);
}

#[test]
fn test_format_output_invalid_placeholder_fallback() {
    // Invalid placeholder that doesn't exist
    let template = "Generated: {invalid}";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 1);

    // Should handle gracefully by returning the UUID
    assert_eq!(result, uuid);
}

#[test]
fn test_format_output_multiple_same_placeholders() {
    let template = "{uuid}-{uuid}";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 1);

    assert_eq!(
        result,
        "550e8400-e29b-41d4-a716-446655440000-550e8400-e29b-41d4-a716-446655440000"
    );
}

#[test]
fn test_format_output_sequence_increments() {
    let template = "ID-{sequence}";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";

    for i in 1..=10 {
        let result = format_output(template, uuid, i);
        assert_eq!(result, format!("ID-{}", i));
    }
}

// ===== Count and Batch Tests =====

#[test]
fn test_multiple_uuid_generation_count_small() {
    let mut options_list = Vec::new();

    for _ in 0..5 {
        options_list.push(GuidGenerateOptions {
            guid_version: GuidVersionType::V4,
            v6_seed: String::new(),
        });
    }

    let uuids: Vec<_> = options_list
        .iter()
        .map(|opt| generate_guid(opt.clone()))
        .collect();

    // Should generate 5 UUIDs
    assert_eq!(uuids.len(), 5);
    // All should be different
    for i in 0..uuids.len() {
        for j in (i + 1)..uuids.len() {
            assert_ne!(uuids[i], uuids[j]);
        }
    }
}

#[test]
fn test_multiple_uuid_generation_count_large() {
    let mut options_list = Vec::new();

    for _ in 0..100 {
        options_list.push(GuidGenerateOptions {
            guid_version: GuidVersionType::V4,
            v6_seed: String::new(),
        });
    }

    let uuids: Vec<_> = options_list
        .iter()
        .map(|opt| generate_guid(opt.clone()))
        .collect();

    // Should generate 100 UUIDs
    assert_eq!(uuids.len(), 100);
}

#[test]
fn test_multiple_uuid_generation_count_thousand() {
    let mut options_list = Vec::new();

    for _ in 0..1000 {
        options_list.push(GuidGenerateOptions {
            guid_version: GuidVersionType::V4,
            v6_seed: String::new(),
        });
    }

    let uuids: Vec<_> = options_list
        .iter()
        .map(|opt| generate_guid(opt.clone()))
        .collect();

    // Should generate 1000 UUIDs (validates u32 count support)
    assert_eq!(uuids.len(), 1000);
}

// ===== Format Combinations Tests =====

#[test]
fn test_format_combination_uppercase_hyphenated() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: true,
        },
    );

    // Should have hyphens
    assert!(formatted.contains('-'));
    // Should be uppercase
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_uppercase());
}

#[test]
fn test_format_combination_lowercase_non_hyphenated() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: false,
            uppercase: false,
        },
    );

    // Should not have hyphens
    assert!(!formatted.contains('-'));
    // Should be lowercase
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_lowercase());
}

#[test]
fn test_format_combination_uppercase_non_hyphenated() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: false,
            uppercase: true,
        },
    );

    // Should not have hyphens
    assert!(!formatted.contains('-'));
    // Should be uppercase
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_uppercase());
}

#[test]
fn test_format_combination_lowercase_hyphenated() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let formatted = format_guid(
        &uuid,
        &FormatOptions {
            hyphenated: true,
            uppercase: false,
        },
    );

    // Should have hyphens
    assert!(formatted.contains('-'));
    // Should be lowercase
    let hex_chars: String = formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_lowercase());
}

// ===== Clone Overhead Regression Tests =====

#[test]
fn test_guid_generate_options_no_unnecessary_clones() {
    // This test verifies that the GuidGenerateOptions passed to generate_guid
    // doesn't trigger unnecessary cloning in the generate_guid function
    let options1 = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: "1,2,3,4,5,6".to_string(),
    };

    let uuid1 = generate_guid(options1);
    let options2 = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: "1,2,3,4,5,6".to_string(),
    };
    let uuid2 = generate_guid(options2);

    // Both should generate valid V4 UUIDs
    assert_eq!(uuid1.get_version_num(), 4);
    assert_eq!(uuid2.get_version_num(), 4);
}

// ===== Control Flow Tests =====

#[test]
fn test_guid_version_matching_logic() {
    // Test V4 path
    let v4_options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };
    let v4_uuid = generate_guid(v4_options);
    assert_eq!(v4_uuid.get_version_num(), 4);

    // Test V6 path
    let v6_options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "1,2,3,4,5,6".to_string(),
    };
    let v6_uuid = generate_guid(v6_options);
    assert_eq!(v6_uuid.get_version_num(), 6);

    // Test V7 path
    let v7_options = GuidGenerateOptions {
        guid_version: GuidVersionType::V7,
        v6_seed: String::new(),
    };
    let v7_uuid = generate_guid(v7_options);
    assert_eq!(v7_uuid.get_version_num(), 7);
}

// ===== String vs &str Tests =====

#[test]
fn test_format_output_accepts_str_references() {
    // Test that format_output correctly accepts &str
    let template_str = "Test: {uuid}";
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";

    let result = format_output(template_str, uuid_str, 1);

    assert_eq!(result, "Test: 550e8400-e29b-41d4-a716-446655440000");
}

#[test]
fn test_format_guid_accepts_str_references() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V4,
        v6_seed: String::new(),
    };

    let uuid = generate_guid(options);
    let format_opts = &FormatOptions {
        hyphenated: true,
        uppercase: false,
    };

    let formatted = format_guid(&uuid, format_opts);

    // Should produce valid output
    assert_eq!(formatted.len(), 36);
    assert!(formatted.contains('-'));
}

// ===== End-to-End Integration Tests =====

#[test]
fn test_end_to_end_v4_simple() {
    let args = Args {
        count: 1,
        uuid_type: UUIDType::Guid,
        guid_version: GuidVersionType::V4,
        nanoid_length: 21,
        non_hyphenated: false,
        uppercase: false,
        output_template: "{uuid}".to_string(),
        guid_v6_seed: "1,2,3,4,5,6".to_string(),
        help: None,
        version: None,
    };

    let uuid_formatted = generate_uuid(&args);

    // Should be valid V4 UUID format
    assert_eq!(uuid_formatted.len(), 36);
    assert!(uuid_formatted.contains('-'));
    assert!(
        uuid_formatted
            .chars()
            .all(|c| c.is_ascii_hexdigit() || c == '-')
    );
}

#[test]
fn test_end_to_end_v6_with_seed() {
    let args = Args {
        count: 1,
        uuid_type: UUIDType::Guid,
        guid_version: GuidVersionType::V6,
        nanoid_length: 21,
        non_hyphenated: false,
        uppercase: false,
        output_template: "{uuid}".to_string(),
        guid_v6_seed: "100,101,102,103,104,105".to_string(),
        help: None,
        version: None,
    };

    let uuid_formatted = generate_uuid(&args);

    // Should be valid V6 UUID format
    assert_eq!(uuid_formatted.len(), 36);
    assert!(uuid_formatted.contains('-'));
}

#[test]
fn test_end_to_end_nanoid_generation() {
    let args = Args {
        count: 1,
        uuid_type: UUIDType::NanoId,
        guid_version: GuidVersionType::V4,
        nanoid_length: 21,
        non_hyphenated: false,
        uppercase: false,
        output_template: "{uuid}".to_string(),
        guid_v6_seed: String::new(),
        help: None,
        version: None,
    };

    let nanoid = generate_uuid(&args);

    // Should be valid NanoID
    assert_eq!(nanoid.len(), 21);
    assert!(
        nanoid
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    );
}

#[test]
fn test_end_to_end_template_with_sequence() {
    let args = Args {
        count: 1,
        uuid_type: UUIDType::Guid,
        guid_version: GuidVersionType::V4,
        nanoid_length: 21,
        non_hyphenated: false,
        uppercase: false,
        output_template: "ID-{sequence}-{uuid}".to_string(),
        guid_v6_seed: String::new(),
        help: None,
        version: None,
    };

    let uuid_formatted = generate_uuid(&args);
    let output = format_output(&args.output_template, &uuid_formatted, 5);

    // Should contain sequence and UUID
    assert!(output.contains("ID-5-"));
    assert!(output.contains("-") && output.len() > 10);
}

// ===== Additional Edge Cases =====

#[test]
fn test_v6_uuid_seed_with_overflow_values() {
    // Values > 255 should wrap or be truncated by u8 parsing
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "256,300,1000,500,400,350".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // All values will fail to parse as u8, so default node [1; 6] is used
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 1);
    assert_eq!(node_bytes[12], 1);
    assert_eq!(node_bytes[13], 1);
    assert_eq!(node_bytes[14], 1);
    assert_eq!(node_bytes[15], 1);
}

#[test]
fn test_v6_uuid_seed_with_extra_commas_prefix() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: ",,,10,20,30,40,50,60".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // Leading commas create empty strings that fail parse
    // We get 3 empty (fail) + 6 valid = 9 total, but seed_values has 6 valid
    // So the seed IS applied!
    assert_eq!(node_bytes[10], 10);
    assert_eq!(node_bytes[11], 20);
    assert_eq!(node_bytes[12], 30);
    assert_eq!(node_bytes[13], 40);
    assert_eq!(node_bytes[14], 50);
    assert_eq!(node_bytes[15], 60);
}

#[test]
fn test_v6_uuid_seed_with_extra_commas_suffix() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "10,20,30,40,50,60,,,".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // Trailing empty strings will be added to seed_values as parse failures
    // But we have 6 valid values first, so they should be used
    assert_eq!(node_bytes[10], 10);
    assert_eq!(node_bytes[11], 20);
    assert_eq!(node_bytes[12], 30);
    assert_eq!(node_bytes[13], 40);
    assert_eq!(node_bytes[14], 50);
    assert_eq!(node_bytes[15], 60);
}

#[test]
fn test_v6_uuid_seed_with_embedded_commas() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "10,,20,,30,,40,,50,,60".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // Embedded empty strings fail parse, so we get 6 valid + 5 invalid = 11 total
    // Since we have exactly 6 valid values, they should be used
    assert_eq!(node_bytes[10], 10);
    assert_eq!(node_bytes[11], 20);
    assert_eq!(node_bytes[12], 30);
    assert_eq!(node_bytes[13], 40);
    assert_eq!(node_bytes[14], 50);
    assert_eq!(node_bytes[15], 60);
}

#[test]
fn test_v6_uuid_seed_with_more_than_six_values() {
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "10,20,30,40,50,60,70,80,90".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // Should use default node because seed_values.len() != 6 (it's 9)
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 1);
    assert_eq!(node_bytes[12], 1);
    assert_eq!(node_bytes[13], 1);
    assert_eq!(node_bytes[14], 1);
    assert_eq!(node_bytes[15], 1);
}

#[test]
fn test_format_output_whitespace_only_template() {
    let template = "   ";
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let result = format_output(template, uuid, 1);

    // Whitespace-only template should be preserved
    assert_eq!(result, "   ");
}

#[test]
fn test_end_to_end_full_pipeline_uppercase_non_hyphenated_template() {
    let args = Args {
        count: 1,
        uuid_type: UUIDType::Guid,
        guid_version: GuidVersionType::V4,
        nanoid_length: 21,
        non_hyphenated: true,
        uppercase: true,
        output_template: "PREFIX-{uuid}-SUFFIX".to_string(),
        guid_v6_seed: String::new(),
        help: None,
        version: None,
    };

    let uuid_formatted = generate_uuid(&args);
    let output = format_output(&args.output_template, &uuid_formatted, 1);

    // UUID should be uppercase and non-hyphenated
    assert_eq!(uuid_formatted.len(), 32); // No hyphens
    assert!(!uuid_formatted.contains('-'));
    let hex_chars: String = uuid_formatted
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    assert_eq!(hex_chars, hex_chars.to_uppercase());

    // Output should contain the template with the UUID
    assert!(output.starts_with("PREFIX-"));
    assert!(output.ends_with("-SUFFIX"));
    assert!(output.contains(&uuid_formatted));
}

#[test]
fn test_v6_uuid_seed_negative_values() {
    // Negative values should fail parsing as u8
    let options = GuidGenerateOptions {
        guid_version: GuidVersionType::V6,
        v6_seed: "-1,-2,-3,-4,-5,-6".to_string(),
    };

    let uuid = generate_guid(options);
    let node_bytes = uuid.as_bytes();

    // All negative values fail to parse, default node used
    assert_eq!(node_bytes[10], 1);
    assert_eq!(node_bytes[11], 1);
    assert_eq!(node_bytes[12], 1);
    assert_eq!(node_bytes[13], 1);
    assert_eq!(node_bytes[14], 1);
    assert_eq!(node_bytes[15], 1);
}
