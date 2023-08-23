use validator::ValidationErrors;
use crate::exceptions::api::ApiException;

impl Into<ApiException> for ValidationErrors {
    fn into(self) -> ApiException {
        let mut nerrors = Vec::new();
        for (field, errors) in self.field_errors() {
            for error in errors {
                nerrors.push(format!("{}: {}", field, error));
            }
        }
        ApiException::ValidationError(nerrors)
    }
}