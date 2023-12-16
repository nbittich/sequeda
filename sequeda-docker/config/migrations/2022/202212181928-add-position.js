const INSERTS = [
  {
    name: "Chief executive office",
    description: `
      A chief executive officer (CEO), also known as a central executive officer (CEO), chief administrator officer (CAO) or just chief executive (CE), 
      is one of a number of corporate executives charged with the management of an organization – especially an independent legal entity such as a company or 
      nonprofit institution.
            `,
    level: "EXECUTIVE",
  },
  {
    name: "Chief financial officer",
    description: `
      The chief financial officer (CFO) is an officer of a company or organization that is assigned the primary responsibility for managing the company's finances, 
      including financial planning, management of financial risks, record-keeping, and financial reporting.
      `,
    level: "EXECUTIVE",
  },
  {
    name: "Chief information officer",
    description: `
      Chief information officer (CIO), chief digital information officer (CDIO) or information technology (IT) director, 
      is a job title commonly given to the most senior executive in an enterprise who works with information technology and computer systems, 
      in order to support enterprise goals.
      `,
    level: "EXECUTIVE",
  },
  {
    name: "Chief marketing officer",
    description: `
      A chief marketing officer (CMO), also called a global marketing officer or marketing director, 
      is a corporate executive responsible for managing marketing activities in an organization.
      `,
    level: "EXECUTIVE",
  },
  {
    name: "Chief operations officer",
    description: `
      A chief operating officer or chief operations officer, also called a COO, is one of the highest-ranking executive positions in an organization, 
      composing part of the "C-suite". 
      The COO is usually the second-in-command at the firm, especially if the highest-ranking executive is the chairperson and CEO.
      `,
    level: "EXECUTIVE",
  },
  {
    name: "Human resources manager",
    description: `
      Human resource management (HRM or HR) is the strategic and coherent approach to the effective and efficient management of people in a
      company or organization such that they help their business gain a competitive advantage. 
      `,
    level: "MANAGEMENT",
  },
  {
    name: "Information technology manager",
    description: `
      Information technology management or IT management is the discipline whereby all of the information technology resources 
      of a firm are managed in accordance with its needs and priorities.
      `,
    level: "MANAGEMENT",
  },
  {
    name: "Marketing manager",
    description: `
      Marketing management is the organizational discipline which focuses on the practical application of marketing orientation, 
      techniques and methods inside enterprises and organizations and on the management of a firm's marketing resources and activities. 
      `,
    level: "MANAGEMENT",
  },
  {
    name: "Product manager",
    description: `
      A product manager (PM) is a professional role that is responsible for the development of products for an organization, µ
      known as the practice of product management. Product managers own the product strategy behind a product (physical or digital), 
      specify its functional requirements, and manage feature releases.
      `,
    level: "MANAGEMENT",
  },
  {
    name: "Sales manager",
    description: `
      Sales management is a business discipline which is focused on the practical application of sales techniques and the management of a firm's sales operations.
      `,
    level: "MANAGEMENT",
  },
  {
    name: "Administrative assistant",
    description: `
      A person responsible for providing various kinds of administrative assistance is called an administrative assistant (admin assistant) 
      or sometimes an administrative support specialist.
      In most instances it is identical to the modern iteration of the position of secretary or is a sub-specialty of secretarial duties. 
      `,
    level: "OPERATIONAL",
  },
  {
    name: "Bookkeeper",
    description: `
      Bookkeeping is the recording of financial transactions, and is part of the process of accounting in business and other organizations. 
      `,
    level: "OPERATIONAL",
  },
  {
    name: "Business analyst",
    description: `
      A business analyst (BA) is a person who processes, interprets and documents business processes, products, services and software through analysis of data.
      The role of a business analyst is to ensure business efficiency increases through their knowledge of both IT and business function.
      `,
    level: "OPERATIONAL",
  },
  {
    name: "Sales representative",
    description: `
      In the case of indirect interaction, a person who sells goods or service on behalf of the owner is known as a salesman or saleswoman or salesperson, 
      but this often refers to someone selling goods in a store/shop, 
      in which case other terms are also common, including salesclerk, shop assistant, and retail clerk.
      `,
    level: "OPERATIONAL",
  },
  {
    name: "Software engineer",
    description: `
      A software engineer is a person who applies the principles of software engineering to design, develop, maintain, test, and evaluate computer software. 
      The term programmer is sometimes used as a synonym, but may also lack connotations of engineering education or skills. 
      `,
    level: "OPERATIONAL",
  },
];

const execute = async (db, context = {}) => {
  const { now, uuid } = context;
  INSERTS.forEach((i) => {
    i.creationDate = now();
    i._id = uuid();
  });
  const positionCollection = await db.collection("position");
  await positionCollection.insertMany(INSERTS);
};

const rollback = async (db, _context = {}) => {
  const positionCollection = await db.collection("position");
  for (const { name } of INSERTS) {
    await positionCollection.deletOne({ name });
  }
};

module.exports = {
  targetDatabases: null, // force to run on all db
  description: "Add default positions",
  rollback,
  execute,
};
