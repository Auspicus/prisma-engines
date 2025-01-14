use crate::composite_type::CompositeType;
use crate::field::RelationField;
use crate::model::Model;
use crate::relation_info::RelationInfo;
use psl_core::schema_ast::ast;

#[derive(Debug, Default)]
pub struct Datamodel {
    pub models: Vec<Model>,
    pub composite_types: Vec<CompositeType>,
}

impl Datamodel {
    /// Gets an iterator over all models.
    pub fn models(&self) -> std::slice::Iter<Model> {
        self.models.iter()
    }

    /// Gets an iterator over all composite types.
    pub fn composite_types(&self) -> std::slice::Iter<CompositeType> {
        self.composite_types.iter()
    }

    /// Finds a model by name.
    pub fn find_model(&self, name: &str) -> Option<&Model> {
        self.models().find(|model| model.name == name)
    }

    pub fn find_model_by_id(&self, id: ast::ModelId) -> Option<&Model> {
        self.models().find(|m| m.id == id)
    }

    /// Finds a composite type by name.
    pub fn find_composite_type(&self, name: &str) -> Option<&CompositeType> {
        self.composite_types().find(|composite| composite.name == name)
    }

    /// Finds a model by database name. This will only find models with a name
    /// remapped to the provided `db_name`.
    pub fn find_model_db_name(&self, db_name: &str) -> Option<&Model> {
        self.models()
            .find(|model| model.database_name.as_deref() == Some(db_name))
    }

    /// Finds a model by name and returns a mutable reference.
    pub fn find_model_mut(&mut self, name: &str) -> &mut Model {
        self.models
            .iter_mut()
            .find(|m| m.name == *name)
            .expect("We assume an internally valid datamodel before mutating.")
    }

    /// Finds a relation field related to a relation info. Returns a tuple (index_of_relation_field_in_model, relation_field).
    pub fn find_related_field_for_info(&self, info: &RelationInfo, exclude: &str) -> Option<(usize, &RelationField)> {
        self.find_model_by_id(info.referenced_model)
            .expect("The model referred to by a RelationInfo should always exist.")
            .fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| field.as_relation_field().map(|f| (idx, f)))
            .find(|(_idx, f)| {
                f.relation_info.name == info.name
                    && (f.relation_info.referenced_model != info.referenced_model ||
          // This is to differentiate the opposite field from self in the self relation case.
          f.name != exclude)
            })
    }

    /// This finds the related field for a relationfield if available
    pub fn find_related_field(&self, rf: &RelationField) -> Option<(usize, &RelationField)> {
        self.find_related_field_for_info(&rf.relation_info, &rf.name)
    }

    /// This is used once we assume the datamodel to be internally valid
    pub fn find_related_field_bang(&self, rf: &RelationField) -> (usize, &RelationField) {
        self.find_related_field(rf)
            .expect("Every RelationInfo should have a complementary RelationInfo on the opposite relation field.")
    }
}
