use validator::ValidationErrors;
use crate::exceptions::api::ApiException;

impl Into<ApiException<'_>> for ValidationErrors {
    fn into(self) -> ApiException<'static> {
        let mut nerrors = Vec::new();
        for (field, errors) in self.field_errors() {
            for error in errors {
                nerrors.push(format!("{}: {}", field, error));
            }
        }
        ApiException::ValidationError("Validation errors", nerrors)
    }
}