use crate::logic::document::driving_ports::PrintListError;

pub struct Document {
    id: String,
    title: String,
}

mod driven_ports {
    use mockall::automock;
    use super::*;

    pub enum ListDocumentsError {
        BadAuth,
        AdapterErr(anyhow::Error)
    }


    #[cfg_attr(test, automock)]
    pub trait DocumentReader {
        // Result::Ok(successful result)
        // Result::Err(error result)
        fn list_documents(&self) -> Result<Vec<Document>, ListDocumentsError>;
    }
}

mod driving_ports{
    use super::*;

    pub enum PrintListError {
        BadAuth,
        NoDocuments,
        AdapterError(anyhow::Error),
    }

    pub trait DocumentDrivingPort {
        fn print_document_list(&self) -> Result<(), PrintListError>;
    }
}

pub struct DocumentService<Rdr>
 where Rdr: driven_ports::DocumentReader {
    doc_reader: Rdr,
}

impl <Rdr> driving_ports::DocumentDrivingPort for DocumentService<Rdr>
    where Rdr: driven_ports::DocumentReader {
    fn print_document_list(&self) -> Result<(), PrintListError> {
        let retrieved_documents_result = self.doc_reader.list_documents();
        let retrieved_documents = match retrieved_documents_result {
            Ok(doc_list) => doc_list,
            Err(driven_ports::ListDocumentsError::BadAuth) => return Err(PrintListError::BadAuth),
            Err(driven_ports::ListDocumentsError::AdapterErr(err)) => return Err(PrintListError::AdapterError(err)),
        };

        if retrieved_documents.is_empty() {
            return Err(PrintListError::NoDocuments);
        }

        println!("List of documents retrieved from GetOutline:");
        for retrieved_document in retrieved_documents.iter() {
            println!("\t* {} - {}", retrieved_document.id, retrieved_document.title);
        }

        Ok(())
    }
}

#[cfg(test)]
mod document_service_tests {
    use crate::logic::document::{Document, DocumentService};
    use crate::logic::document::driven_ports::MockDocumentReader;
    use crate::logic::document::driving_ports::DocumentDrivingPort;

    #[test]
    fn can_successfully_print_documents() {
        let mut driven_port = MockDocumentReader::new();
        driven_port.expect_list_documents().returning(|| {
            Ok(vec![
                Document {
                    id: String::from("12345"),
                    title: String::from("My cool document"),
                }
            ])
        });

        let doc_service = DocumentService {
            doc_reader: driven_port,
        };

        let result = doc_service.print_document_list();
        assert!(result.is_ok());
    }
}

