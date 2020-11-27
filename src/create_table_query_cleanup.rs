extern crate regex;
use regex::Regex;

pub fn remove_foreign_keys_constraints(create_table_query: &String) -> String {
  let rg = Regex::new(
    r"(,\n)?[\s\t]*CONSTRAINT\s`\w+`\sFOREIGN\sKEY\s\([`\w+`(,\s)?]+\)\sREFERENCES\s`\w+`\s\([`\w+`(, )?]+\),?",
  )
  .unwrap();
  let cleaned_create_table_query = rg.replace_all(create_table_query, "");
  String::from(cleaned_create_table_query)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_no_foreign_key_constraint() {
    let query = String::from(
      "CREATE TABLE `budgetary_exercises` (
      `id` char(36) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
      `company_id` varchar(14) NOT NULL,
      `name` varchar(255) NOT NULL,
      `start_date` datetime NOT NULL,
      `end_date` datetime NOT NULL,
      `periodicity` enum('yearly','monthly','quarterly') NOT NULL,
      `created_at` datetime NOT NULL,
      `updated_at` datetime NOT NULL,
      `deleted_at` datetime DEFAULT NULL,
      PRIMARY KEY (`id`),
      KEY `budgetary_exercises_company_id` (`company_id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8",
    );

    assert_eq!(super::remove_foreign_keys_constraints(&query), query);
  }

  #[test]
  fn test_one_foreign_key_constraint_composed() {
    let query = String::from("CREATE TABLE `ui_session_states` (
      `id` varchar(14) NOT NULL,
      `domain` varchar(255) NOT NULL,
      `key` varchar(255) NOT NULL,
      `data` json DEFAULT NULL,
      `created_at` datetime DEFAULT NULL,
      `updated_at` datetime DEFAULT NULL,
      PRIMARY KEY (`id`),
      KEY `ui_session_states_updated_at` (`updated_at`),
      KEY `ui_session_states_fk` (`key`,`domain`),
      CONSTRAINT `ui_session_states_fk` FOREIGN KEY (`key`, `domain`) REFERENCES `ui_master_states` (`key`, `domain`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8");

    let cleaned_query = String::from(
      "CREATE TABLE `ui_session_states` (
      `id` varchar(14) NOT NULL,
      `domain` varchar(255) NOT NULL,
      `key` varchar(255) NOT NULL,
      `data` json DEFAULT NULL,
      `created_at` datetime DEFAULT NULL,
      `updated_at` datetime DEFAULT NULL,
      PRIMARY KEY (`id`),
      KEY `ui_session_states_updated_at` (`updated_at`),
      KEY `ui_session_states_fk` (`key`,`domain`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8",
    );

    assert_eq!(
      super::remove_foreign_keys_constraints(&query),
      cleaned_query
    );
  }

  #[test]
  fn test_one_foreign_key_constraint() {
    let query = String::from("CREATE TABLE `budgets` (
      `id` char(36) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
      `company_id` varchar(14) NOT NULL,
      `budgetary_exercise_id` char(36) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
      `parent_budget_id` char(36) CHARACTER SET utf8 COLLATE utf8_bin DEFAULT NULL,
      `currency` varchar(3) NOT NULL,
      `created_at` datetime NOT NULL,
      `updated_at` datetime NOT NULL,
      `deleted_at` datetime DEFAULT NULL,
      `responsibility_center_id` char(36) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
      PRIMARY KEY (`id`,`budgetary_exercise_id`),
      KEY `budgets_company_id` (`company_id`),
      KEY `budgets_parent_budget_id` (`parent_budget_id`),
      KEY `budgets_budgetary_exercise_id` (`budgetary_exercise_id`),
      CONSTRAINT `budgets_ibfk_1` FOREIGN KEY (`budgetary_exercise_id`) REFERENCES `budgetary_exercises` (`id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8");

    let cleaned_query = String::from(
      "CREATE TABLE `budgets` (
      `id` char(36) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
      `company_id` varchar(14) NOT NULL,
      `budgetary_exercise_id` char(36) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
      `parent_budget_id` char(36) CHARACTER SET utf8 COLLATE utf8_bin DEFAULT NULL,
      `currency` varchar(3) NOT NULL,
      `created_at` datetime NOT NULL,
      `updated_at` datetime NOT NULL,
      `deleted_at` datetime DEFAULT NULL,
      `responsibility_center_id` char(36) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL,
      PRIMARY KEY (`id`,`budgetary_exercise_id`),
      KEY `budgets_company_id` (`company_id`),
      KEY `budgets_parent_budget_id` (`parent_budget_id`),
      KEY `budgets_budgetary_exercise_id` (`budgetary_exercise_id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8",
    );

    assert_eq!(
      super::remove_foreign_keys_constraints(&query),
      cleaned_query
    );
  }

  #[test]
  fn test_several_foreign_keys_constraints() {
    let query = String::from("CREATE TABLE `payments` (
      `id` varchar(14) NOT NULL,
      `index` int(11) NOT NULL AUTO_INCREMENT,
      `request_id` varchar(14) DEFAULT NULL,
      `company_id` varchar(14) DEFAULT NULL,
      `user_id` varchar(14) DEFAULT NULL,
      `card_id` varchar(14) DEFAULT NULL,
      `subscription_id` varchar(14) DEFAULT NULL,
      `state` enum('authorised','settled','cancelled','reimbursed') NOT NULL,
      `method` enum('card','cash','other','transfer') DEFAULT NULL,
      `currency_declared` varchar(3) DEFAULT NULL,
      `amount_declared` decimal(10,2) DEFAULT NULL,
      `amount_billed` decimal(10,2) DEFAULT NULL,
      `fx_fee_amount` decimal(10,2) DEFAULT NULL,
      `paid_at` datetime DEFAULT NULL,
      `last_reminder_at` datetime DEFAULT NULL,
      `created_at` datetime DEFAULT NULL,
      `updated_at` datetime DEFAULT NULL,
      `deleted_at` datetime DEFAULT NULL,
      `parent_id` varchar(14) DEFAULT NULL,
      `description` varchar(255) CHARACTER SET utf8mb4 DEFAULT NULL,
      `supplier_id` varchar(14) DEFAULT NULL,
      `group_id` varchar(14) DEFAULT NULL,
      `invoice_lost` tinyint(1) NOT NULL DEFAULT '0',
      `particularity` enum('visa_support','atm_withdrawal') DEFAULT NULL,
      `nature_id` int(11) DEFAULT NULL,
      `organisation_id` varchar(14) DEFAULT NULL,
      `is_brand_verified` tinyint(1) NOT NULL DEFAULT '0',
      `accounting_code` varchar(255) DEFAULT NULL,
      `invoice_number` varchar(255) DEFAULT NULL,
      `vat_type` enum('liable','reverse_charge','not_liable') DEFAULT NULL,
      `synced_at` datetime DEFAULT NULL,
      `provider_invoice_id` varchar(255) DEFAULT NULL,
      `reminded_counter` int(11) NOT NULL DEFAULT '0',
      `accounted_at` datetime DEFAULT NULL,
      `validated_at` datetime DEFAULT NULL,
      `read_at` datetime DEFAULT NULL,
      `reimbursed_at` datetime DEFAULT NULL,
      `validated_by` varchar(14) DEFAULT NULL,
      `accounted_by` varchar(14) DEFAULT NULL,
      `expense_account_id` varchar(14) DEFAULT NULL,
      `auto_matched_expense_account_with` enum('category','supplier') DEFAULT NULL,
      `guessed_supplier_id` varchar(14) DEFAULT NULL,
      `version` int(10) unsigned NOT NULL DEFAULT '0',
      `invoice_invalid` tinyint(1) NOT NULL DEFAULT '0',
      `vat_manual_verification_status` enum('pending','success') DEFAULT NULL,
      `invoice_date` date DEFAULT NULL,
      `receipt_number` varchar(36) DEFAULT NULL,
      `employee_account_id` varchar(14) DEFAULT NULL,
      `supplier_account_id` varchar(14) DEFAULT NULL,
      `invoice_invalid_reason` varchar(255) DEFAULT NULL,
      PRIMARY KEY (`id`),
      UNIQUE KEY `index` (`index`),
      KEY `payments_company_id` (`company_id`),
      KEY `payments_request_id` (`request_id`),
      KEY `payments_user_id` (`user_id`),
      KEY `payments_card_id` (`card_id`),
      KEY `payments_subscription_id` (`subscription_id`),
      KEY `payments_state` (`state`),
      KEY `payments_parent_id` (`parent_id`),
      KEY `payments_supplier_id` (`supplier_id`),
      KEY `payments_group_id` (`group_id`),
      KEY `payments_category_id_foreign_idx` (`nature_id`),
      KEY `payments_validated_by_foreign_idx` (`validated_by`),
      KEY `payments_accounted_by_foreign_idx` (`accounted_by`),
      KEY `payments_expense_account_id` (`expense_account_id`),
      KEY `payments_guessed_supplier_id` (`guessed_supplier_id`),
      KEY `payments_employee_account_id_foreign_idx` (`employee_account_id`),
      KEY `payments_supplier_account_id_foreign_idx` (`supplier_account_id`),
      KEY `payments_invoice_number` (`invoice_number`),
      CONSTRAINT `payments_accounted_by_foreign_idx` FOREIGN KEY (`accounted_by`) REFERENCES `users` (`id`),
      CONSTRAINT `payments_category_id_foreign_idx` FOREIGN KEY (`nature_id`) REFERENCES `natures` (`id`),
      CONSTRAINT `payments_employee_account_id_foreign_idx` FOREIGN KEY (`employee_account_id`) REFERENCES `employee_accounts` (`id`),
      CONSTRAINT `payments_expense_account_id_foreign_idx` FOREIGN KEY (`expense_account_id`) REFERENCES `expense_accounts` (`id`),
      CONSTRAINT `payments_supplier_account_id_foreign_idx` FOREIGN KEY (`supplier_account_id`) REFERENCES `supplier_accounts` (`id`),
      CONSTRAINT `payments_validated_by_foreign_idx` FOREIGN KEY (`validated_by`) REFERENCES `users` (`id`)
    ) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8");

    let cleaned_query = String::from(
      "CREATE TABLE `payments` (
      `id` varchar(14) NOT NULL,
      `index` int(11) NOT NULL AUTO_INCREMENT,
      `request_id` varchar(14) DEFAULT NULL,
      `company_id` varchar(14) DEFAULT NULL,
      `user_id` varchar(14) DEFAULT NULL,
      `card_id` varchar(14) DEFAULT NULL,
      `subscription_id` varchar(14) DEFAULT NULL,
      `state` enum('authorised','settled','cancelled','reimbursed') NOT NULL,
      `method` enum('card','cash','other','transfer') DEFAULT NULL,
      `currency_declared` varchar(3) DEFAULT NULL,
      `amount_declared` decimal(10,2) DEFAULT NULL,
      `amount_billed` decimal(10,2) DEFAULT NULL,
      `fx_fee_amount` decimal(10,2) DEFAULT NULL,
      `paid_at` datetime DEFAULT NULL,
      `last_reminder_at` datetime DEFAULT NULL,
      `created_at` datetime DEFAULT NULL,
      `updated_at` datetime DEFAULT NULL,
      `deleted_at` datetime DEFAULT NULL,
      `parent_id` varchar(14) DEFAULT NULL,
      `description` varchar(255) CHARACTER SET utf8mb4 DEFAULT NULL,
      `supplier_id` varchar(14) DEFAULT NULL,
      `group_id` varchar(14) DEFAULT NULL,
      `invoice_lost` tinyint(1) NOT NULL DEFAULT '0',
      `particularity` enum('visa_support','atm_withdrawal') DEFAULT NULL,
      `nature_id` int(11) DEFAULT NULL,
      `organisation_id` varchar(14) DEFAULT NULL,
      `is_brand_verified` tinyint(1) NOT NULL DEFAULT '0',
      `accounting_code` varchar(255) DEFAULT NULL,
      `invoice_number` varchar(255) DEFAULT NULL,
      `vat_type` enum('liable','reverse_charge','not_liable') DEFAULT NULL,
      `synced_at` datetime DEFAULT NULL,
      `provider_invoice_id` varchar(255) DEFAULT NULL,
      `reminded_counter` int(11) NOT NULL DEFAULT '0',
      `accounted_at` datetime DEFAULT NULL,
      `validated_at` datetime DEFAULT NULL,
      `read_at` datetime DEFAULT NULL,
      `reimbursed_at` datetime DEFAULT NULL,
      `validated_by` varchar(14) DEFAULT NULL,
      `accounted_by` varchar(14) DEFAULT NULL,
      `expense_account_id` varchar(14) DEFAULT NULL,
      `auto_matched_expense_account_with` enum('category','supplier') DEFAULT NULL,
      `guessed_supplier_id` varchar(14) DEFAULT NULL,
      `version` int(10) unsigned NOT NULL DEFAULT '0',
      `invoice_invalid` tinyint(1) NOT NULL DEFAULT '0',
      `vat_manual_verification_status` enum('pending','success') DEFAULT NULL,
      `invoice_date` date DEFAULT NULL,
      `receipt_number` varchar(36) DEFAULT NULL,
      `employee_account_id` varchar(14) DEFAULT NULL,
      `supplier_account_id` varchar(14) DEFAULT NULL,
      `invoice_invalid_reason` varchar(255) DEFAULT NULL,
      PRIMARY KEY (`id`),
      UNIQUE KEY `index` (`index`),
      KEY `payments_company_id` (`company_id`),
      KEY `payments_request_id` (`request_id`),
      KEY `payments_user_id` (`user_id`),
      KEY `payments_card_id` (`card_id`),
      KEY `payments_subscription_id` (`subscription_id`),
      KEY `payments_state` (`state`),
      KEY `payments_parent_id` (`parent_id`),
      KEY `payments_supplier_id` (`supplier_id`),
      KEY `payments_group_id` (`group_id`),
      KEY `payments_category_id_foreign_idx` (`nature_id`),
      KEY `payments_validated_by_foreign_idx` (`validated_by`),
      KEY `payments_accounted_by_foreign_idx` (`accounted_by`),
      KEY `payments_expense_account_id` (`expense_account_id`),
      KEY `payments_guessed_supplier_id` (`guessed_supplier_id`),
      KEY `payments_employee_account_id_foreign_idx` (`employee_account_id`),
      KEY `payments_supplier_account_id_foreign_idx` (`supplier_account_id`),
      KEY `payments_invoice_number` (`invoice_number`)
    ) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8",
    );

    assert_eq!(
      super::remove_foreign_keys_constraints(&query),
      cleaned_query
    );
  }
}
